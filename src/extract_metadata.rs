use crate::date_time::DateTime;
use std::convert::From;
use std::error;
use std::fmt;
use std::fs::File;
use std::path::Path;
use rexiv2::Metadata;

#[derive(Debug)]
pub struct PhotoMetadata {
    pub datetime: DateTime,
}

type Result<T> = std::result::Result<T, MetadataExtractorError>;

impl PhotoMetadata {
    pub fn from_file(file_path: &Path) -> Result<PhotoMetadata> {
        PhotoMetadata::from_exif(&file_path)
    }

    fn from_exif(file_path: &Path) -> Result<PhotoMetadata> {
        let file = File::open(file_path)?;

        if let Ok(meta) = Metadata::new_from_path(file_path.to_str().unwrap()) {
            if let Ok(datetime_ascii) = meta.get_tag_string("Exif.Image.DateTime") {
                let datetime = DateTime::from_ascii(datetime_ascii.as_bytes()).unwrap();
                return Ok(PhotoMetadata { datetime })
            }
        }

        Err(MetadataExtractorError {})
    }
}

#[derive(Debug, Clone)]
pub struct MetadataExtractorError;

impl fmt::Display for MetadataExtractorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not extract metadata from file")
    }
}

impl error::Error for MetadataExtractorError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<std::io::Error> for MetadataExtractorError {
    fn from(_error: std::io::Error) -> Self {
        MetadataExtractorError {}
    }
}

impl From<exif::Error> for MetadataExtractorError {
    fn from(_error: exif::Error) -> Self {
        MetadataExtractorError {}
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
        assert_eq!(metadata.datetime.year, 2019);
        assert_eq!(metadata.datetime.month, 8);
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
