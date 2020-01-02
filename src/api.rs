use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
enum Request {
    #[serde(rename = "START_PROCESSING")]
    StartProcessing { args: StartProcessingArgs },

    #[serde(rename = "READ_EXIF_DATA")]
    ReadExifData { args: ReadExifDataArgs },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "id")]
enum Response {
    #[serde(rename = "PROCESSING_STARTED")]
    ProcessingStarted,

    #[serde(rename = "PROCESSING_PROGRESS")]
    ProcessingProgress { payload: ProcessingProgressPayload },

    #[serde(rename = "PROCESSING_FINISHED")]
    ProcessingFinished,

    #[serde(rename = "EXIF_DATA")]
    ExifData { payload: ExifDataPayload },
}

#[derive(Serialize, Deserialize, Debug)]
struct StartProcessingArgs {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReadExifDataArgs {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProcessingProgressPayload {
    progress: u32,
    total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExifDataPayload {
    exif_data: Value,
}

pub fn process_from_json_string(input: &str) {
    let req: Request = serde_json::from_str(input).unwrap();

    match req {
        Request::StartProcessing { args } => {
            send_response(Response::ProcessingStarted);
            pastelogue::process_dir(&PathBuf::from(&args.path));
            send_response(Response::ProcessingFinished);
        },
        Request::ReadExifData { .. } => {
            todo!("Action READ_EXIF_DATA is not implemented");
        }
    }
}

fn send_response(res: Response) {
    let json = serde_json::to_string(&res).unwrap();
    println!("{}", &json);
}
