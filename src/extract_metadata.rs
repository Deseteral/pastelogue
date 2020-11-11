use crate::date_time::ExifDateTime;
use crate::exiv2;
use std::path::Path;

#[derive(Debug)]
pub struct PhotoMetadata {
    pub datetime: ExifDateTime,
}

impl PhotoMetadata {
    pub fn from_file(file_path: &Path) -> Result<PhotoMetadata, exiv2::ExifReadError> {
        let metadata = exiv2::read_metadata_from_file(file_path)?;
        let date_time_str = &metadata
            .get("Exif.Photo.DateTimeOriginal") // TODO: First of all this should fallback to DateTime. But! There's also Exif.Image.DateTimeOriginal - which one should be used?
            .ok_or_else(|| exiv2::ExifReadError {})?
            .as_str();

        Ok(PhotoMetadata {
            datetime: ExifDateTime::from_exif_string(date_time_str).unwrap(),
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
        assert_eq!(metadata.datetime.year, "2019"); // TODO: Create custom assertion like this:
        assert_eq!(metadata.datetime.month, "08"); //       `assert_datetime_eq!(metadata.datetime, 2019, 12, 10, 13, 30)`
        assert_eq!(metadata.datetime.day, "04");

        assert_eq!(metadata.datetime.hour, "15");
        assert_eq!(metadata.datetime.minute, "21");
        assert_eq!(metadata.datetime.second, "20");
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
