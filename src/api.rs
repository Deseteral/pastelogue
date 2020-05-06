use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json::{Value};
use pastelogue::{CatalogueProcessor, ProcessingStatus};

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
    #[serde(rename = "READY")]
    Ready { payload: ReadyPayload },

    #[serde(rename = "PROCESSING_STARTED")]
    ProcessingStarted,

    #[serde(rename = "PROCESSING_PROGRESS")]
    ProcessingProgress { payload: ProcessingProgressPayload },

    #[serde(rename = "PROCESSING_FINISHED")]
    ProcessingFinished,

    #[serde(rename = "EXIF_DATA")]
    ExifData { payload: ExifDataPayload },

    #[serde(rename = "ERROR")]
    Error { payload: ErrorPayload },
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
struct ReadyPayload {
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProcessingProgressPayload {
    progress: ProcessingProgress,
    file: FileProcessing,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProcessingProgress {
    current: u32,
    total: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileProcessing {
    input: FileInfo,
    output: FileInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ExifDataPayload {
    exif_data: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorPayload {
    messages: Vec<String>,
}

pub fn server_started() {
    let payload = ReadyPayload {
        version: env!("CARGO_PKG_VERSION").to_owned(),
    };
    send_response(Response::Ready { payload });
}

pub fn process_from_json_string(input: &str) {
    let req: Request = serde_json::from_str(input).unwrap();

    match req {
        Request::StartProcessing { args } => {
            send_response(Response::ProcessingStarted);

            let path = PathBuf::from(&args.path);
            let catalogue_processor = CatalogueProcessor::new(&path);

            for processing_info in catalogue_processor {
                if processing_info.status == ProcessingStatus::BadMetadata {
                    let error_message = format!(
                        "File {} has malformed or missing metadata",
                        processing_info.path.display()
                    );
                    let payload = ErrorPayload { messages: vec!(error_message) };
                    send_response(Response::Error { payload });
                }

                let payload = ProcessingProgressPayload {
                    progress: ProcessingProgress {
                        current: processing_info.current,
                        total: processing_info.total,
                    },
                    file: FileProcessing {
                        input: FileInfo {
                            path: processing_info.original_path,
                        },
                        output: FileInfo {
                            path: processing_info.path,
                        }
                    }
                };
                send_response(Response::ProcessingProgress { payload });
            }

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
