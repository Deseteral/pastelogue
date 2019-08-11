use std::io::BufReader;
use std::fs::File;
use exif::{DateTime, Reader, Value, Tag};

pub struct PhotoMetadata {
    pub datetime: DateTime,
}

impl PhotoMetadata {
    pub fn from_file(file_path: &str) -> PhotoMetadata {
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
        let path_str = path.to_str().unwrap();

        // when
        let metadata = PhotoMetadata::from_file(path_str);

        // then
        assert_eq!(metadata.datetime.year, 2019);
        assert_eq!(metadata.datetime.month, 8);
        assert_eq!(metadata.datetime.day, 4);

        assert_eq!(metadata.datetime.hour, 15);
        assert_eq!(metadata.datetime.minute, 21);
        assert_eq!(metadata.datetime.second, 20);
    }
}
