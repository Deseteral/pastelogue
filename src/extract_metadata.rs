use crate::date_time::DateTime;
use std::path::Path;
use crate::exiv2;

#[derive(Debug)]
pub struct PhotoMetadata {
    pub datetime: DateTime,
}

impl PhotoMetadata {
    pub fn from_file(file_path: &Path) -> Result<PhotoMetadata, exiv2::ExifReadError> {
        let metadata = exiv2::read_metadata_from_file(file_path)?;
        let date_time_str = &metadata["Exif"]["Image"]["DateTime"]
            .as_str()
            .ok_or_else(|| exiv2::ExifReadError {})?;

        Ok(PhotoMetadata {
            datetime: DateTime::from_ascii(date_time_str.as_bytes()).unwrap(),
        })
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
        let metadata = PhotoMetadata::from_file(&path).unwrap();

        // then
        assert_eq!(metadata.datetime.year, 2019); // TODO: Create custom assertion like this:
        assert_eq!(metadata.datetime.month, 8);  //       `assert_datetime_eq!(metadata.datetime, 2019, 12, 10, 13, 30)`
        assert_eq!(metadata.datetime.day, 4);

        assert_eq!(metadata.datetime.hour, 15);
        assert_eq!(metadata.datetime.minute, 21);
        assert_eq!(metadata.datetime.second, 20);
    }

    #[test]
    fn it_should_read_metadata_from_file_without_exif() {
        // TODO
    }

    #[test]
    fn it_should_fail_on_invalid_file() {
        // TODO
    }
}
