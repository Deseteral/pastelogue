use std::collections::HashMap;
use std::convert::From;
use std::env;
use std::error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn read_metadata_from_file(file_path: &Path) -> Result<HashMap<String, String>, ExifReadError> {
    let exec_path = get_exiv2_path();
    let exiv2_output = Command::new(exec_path)
        .arg("-PEkv")
        .arg(file_path.to_str().unwrap())
        .output()
        .expect("failed to execute exiv2json process");

    let output_str = String::from_utf8(exiv2_output.stdout)?;

    Ok(process_exiv2_output(&output_str))
}

fn get_exiv2_path() -> PathBuf {
    let base_path = if let Ok(cargo_manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(cargo_manifest_dir)
            .join("release")
            .join("exiv2")
    } else {
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .canonicalize()
            .unwrap() // TODO: Simplify this
    };

    base_path.join("exiv2")
}

fn process_exiv2_output(output_str: &String) -> HashMap<String, String> {
    let mut data: HashMap<String, String> = HashMap::new();

    output_str.split('\n').for_each(|line| {
        let tokens: Vec<&str> = line.split(' ').collect();
        let key: String = String::from(tokens[0].trim());
        let value: String = String::from(tokens[1..tokens.len()].join(" ").trim());

        if !key.is_empty() {
            data.insert(key, value);
        }
    });

    data
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_build_exif_data_from_exiv2_output() {
        // given
        let exiv2_output = String::from("Exif.Photo.DateTimeOriginal                   2019:08:04 15:21:20\nExif.GPSInfo.GPSLatitude                      52/1 24/1 46123/10000");

        // when
        let data = process_exiv2_output(&exiv2_output);

        // then
        assert_eq!(
            data.get("Exif.GPSInfo.GPSLatitude").unwrap(),
            "52/1 24/1 46123/10000",
        );
        assert_eq!(
            data.get("Exif.Photo.DateTimeOriginal").unwrap(),
            "2019:08:04 15:21:20",
        );
    }
}
