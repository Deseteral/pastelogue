mod integration {
    use std::{io::Write, process::*};

    #[test]
    fn it_should_process_catalogue_with_correct_output() {
        // given
        let mut child = Command::new("./target/debug/pastelogue_server")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
            // .output()
            // .expect("failed to execute process");

        let start_processing_json = r#"{"action": "START_PROCESSING", "args": { "path": "./resources/test" } }"#;

        // when
        {
            let child_stdin = child.stdin.as_mut().unwrap();
            child_stdin.write(format!("{}\n", &start_processing_json).as_bytes()).unwrap();
        }
        let output = child.wait_with_output().unwrap();
        let output_str = String::from_utf8(output.stdout).unwrap();

        // then
        let output_lines: Vec<&str> = output_str.split("\n").filter(|s| !s.is_empty()).collect();
        assert_eq!(output_lines, [
            r#"{"id":"PROCESSING_STARTED"}"#,
            r#"{"id":"PROCESSING_PROGRESS","payload":{"progress":1,"total":1,"path":"./resources/test/IMG_20190804_152120.jpg"}}"#,
            r#"{"id":"PROCESSING_FINISHED"}"#,
        ]);
    }
}
