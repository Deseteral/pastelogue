use rustyline::Editor;

mod api;

fn main() {
    let mut rl = Editor::<()>::new();

    api::server_started();

    loop {
        let readline = rl.readline("");
        match readline {
            Ok(line) => {
                api::process_from_json_string(&line);
            }
            Err(_err) => {
                break; // TODO: Add proper error handling
            }
        }
    }
}
