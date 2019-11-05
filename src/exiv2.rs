use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;
use std::convert::From;
use std::error;
use std::fmt;
use serde_json;
use regex::Regex;

pub fn read_metadata_from_file(file_path: &Path) -> Result<serde_json::Value, ExifReadError> {
    let exec_path = get_exiv2json_path();
    dbg!(&exec_path);
    let exiv2json_output = Command::new(exec_path)
        .arg(file_path.to_str().unwrap())
        .output()
        .expect("failed to execute exiv2json process");

    dbg!(&exiv2json_output);

    let output_str = String::from_utf8(exiv2json_output.stdout)?;
    let json = remove_control_characters(&output_str);

    serde_json::from_str(&json).map_err(|_err| ExifReadError {})
}

fn get_exiv2json_path() -> PathBuf {
    let base_path = if let Ok(cargo_manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(cargo_manifest_dir).join("build").join("exiv2")
    } else {
        env::current_exe().unwrap()
            .parent().unwrap()
            .canonicalize().unwrap() // TODO: Simplify this
    };

    base_path.join("exiv2json")
}

fn remove_control_characters(output: &str) -> String {
    let pattern = Regex::new(r"\p{Cc}").unwrap();
    pattern.replace_all(&output, "").to_string()
}


#[derive(Debug, Clone)]
pub struct ExifReadError;

impl fmt::Display for ExifReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not extract metadata from file")
    }
}

impl error::Error for ExifReadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<std::string::FromUtf8Error> for ExifReadError {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        ExifReadError {}
    }
}
