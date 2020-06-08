mod integration {
    use std::{io::Write, process::*};
    use fs_extra::dir;

    #[test]
    fn it_should_process_catalogue_with_correct_output() {
        // setup
        cleanup();
        setup();

        // given
        let mut process = spawn_pastelogue_server();

        // when
        let start_processing_json = r#"{"action": "START_PROCESSING", "args": { "path": "./resources/it_test" } }"#;
        write_line_to_process(start_processing_json, &mut process);

        // then
        let output_lines = get_process_output_lines(process);
        assert_eq!(output_lines, [
            r#"{"id":"READY","payload":{"version":"0.4.0"}}"#,
            r#"{"id":"PROCESSING_STARTED"}"#,
            r#"{"id":"PROCESSING_PROGRESS","payload":{"progress":{"current":1,"total":1},"file":{"input":{"path":"./resources/it_test/IMG_20190804_152120.jpg"},"output":{"path":"./resources/it_test/2019/08/04/2019-08-04_15-21-20.jpg"}},"metadata":{"date":"2019-08-04T15:21:20"}}}"#,
            r#"{"id":"PROCESSING_FINISHED"}"#,
        ]);

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
        process_stdin.write(format!("{}\n", line).as_bytes()).unwrap();
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
