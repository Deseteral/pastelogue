mod integration {
    use fs_extra::dir;
    use path_slash::PathExt;
    use serde_json::{json, Value};
    use std::path::Path;
    use std::{io::Write, process::*};

    #[test]
    fn it_should_process_catalogue_with_correct_output() {
        // setup
        cleanup();
        setup();

        // given
        let mut process = spawn_pastelogue_server();

        // when
        let start_processing_json = json!({
            "action": "START_PROCESSING",
            "args": { "path": Path::new("./resources/it_test").to_slash().unwrap() }
        })
        .to_string();

        write_line_to_process(&start_processing_json, &mut process);

        // then
        let output_lines: Vec<Value> = get_process_output_lines(process)
            .into_iter()
            .map(|s| serde_json::from_str(&s).unwrap())
            .collect();

        let expected_lines: Vec<Value> = vec![
            json!({
                "id": "READY",
                "payload": { "version": "0.6.0" }
            }),
            json!({ "id": "PROCESSING_STARTED" }),
            json!({
                "id": "PROCESSING_PROGRESS",
                "payload": {
                    "progress": {
                        "current": 1,
                        "total": 1
                    },
                    "file": {
                        "input": {
                            "path": Path::new("./resources/it_test/IMG_20190804_152120.jpg").to_slash().unwrap()
                        },
                        "output": {
                            "path": Path::new("./resources/it_test/2019/08/04/2019-08-04_15-21-20.jpg").to_slash().unwrap()
                        }
                    },
                    "metadata": {
                        "createdAt": "2019-08-04T15:21:20Z"
                    }
                }
            }),
            json!({ "id": "PROCESSING_FINISHED" }),
        ];

        assert_eq!(output_lines, expected_lines);

        // cleanup
        cleanup();
    }

    fn setup() {
        let mut opts = dir::CopyOptions::new();
        opts.copy_inside = true;
        dir::copy("./resources/test", "./resources/it_test", &opts).unwrap();
    }

    fn cleanup() {
        dir::remove("resources/it_test").unwrap();
    }

    fn spawn_pastelogue_server() -> Child {
        Command::new("./target/debug/pastelogue_server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }

    fn write_line_to_process(line: &str, process: &mut Child) {
        let process_stdin = process.stdin.as_mut().unwrap();
        process_stdin
            .write(format!("{}\n", line).as_bytes())
            .unwrap();
    }

    fn get_process_output_lines(process: Child) -> Vec<String> {
        let output = process.wait_with_output().unwrap();
        let output_str = String::from_utf8(output.stdout).unwrap();
        let output_lines: Vec<String> = output_str
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect();

        output_lines
    }
}
