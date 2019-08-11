use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use exif::{DateTime, Reader, Value, Tag};

#[derive(Debug)]
pub struct PhotoMetadata {
    pub datetime: DateTime,
}

impl PhotoMetadata {
    pub fn from_file(file_path: &Path) -> PhotoMetadata {
        let file = File::open(file_path).unwrap();
        let reader = Reader::new(&mut BufReader::new(&file)).unwrap();

        let field = reader.get_field(Tag::DateTime, false).unwrap();
        let data = match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => vec[0],
            _ => panic!(), // TODO: This should not panic, add error handling
        };
        let datetime = DateTime::from_ascii(data).unwrap();

        PhotoMetadata { datetime }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_should_read_metadata() {
        // given
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("test")
            .join("IMG_20190804_152120.jpg");

        // when
        let metadata = PhotoMetadata::from_file(&path);

        // then
        assert_eq!(metadata.datetime.year, 2019);
        assert_eq!(metadata.datetime.month, 8);
        assert_eq!(metadata.datetime.day, 4);

        assert_eq!(metadata.datetime.hour, 15);
        assert_eq!(metadata.datetime.minute, 21);
        assert_eq!(metadata.datetime.second, 20);
    }
}
