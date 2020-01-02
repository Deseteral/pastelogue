use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "action")]
enum Request {
    #[serde(rename = "START_PROCESSING")]
    StartProcessing { path: String },

    #[serde(rename = "READ_EXIF_DATA")]
    ReadExifData { path: String },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "id")]
enum Response {
    #[serde(rename = "PROCESSING_STARTED")]
    ProcessingStarted,

    #[serde(rename = "PROCESSING_PROGRESS")]
    ProcessingProgress { progress: u32, total: u32 },

    #[serde(rename = "PROCESSING_FINISHED")]
    ProcessingFinished,

    #[serde(rename = "EXIF_DATA")]
    ExifData { exif_data: Value },
}

pub fn process_from_json_string(input: &str) {
    let req: Request = serde_json::from_str(input).unwrap();
    dbg!(&req);
}
