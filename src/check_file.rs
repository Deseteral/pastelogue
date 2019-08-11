use std::path::{Path, PathBuf};
use exif::DateTime;
use crate::extract_metadata::PhotoMetadata;

#[derive(PartialEq, Debug)]
pub enum CheckStatus {
    Correct,
    Wrong,
}

fn generate_desired_filename(metadata: &PhotoMetadata, extension: &str) -> String {
    format!(
        "{:04}-{:0>2}-{:0>2}_{:0>2}-{:0>2}-{:0>2}.{}",
        metadata.datetime.year,
        metadata.datetime.month,
        metadata.datetime.day,
        metadata.datetime.hour,
        metadata.datetime.minute,
        metadata.datetime.second,
        extension,
    )
}

fn generate_desired_directory_path(metadata: &PhotoMetadata) -> PathBuf {
    Path::new("")
        .join(format!("{:04}", metadata.datetime.year))
        .join(format!("{:0>2}", metadata.datetime.month))
        .join(format!("{:0>2}", metadata.datetime.day))
}

pub fn check_file(file_path: &Path, metadata: PhotoMetadata, root_path: &Path) -> CheckStatus {
    let relative_path = file_path.strip_prefix(root_path).unwrap();
    let extenstion = file_path.extension().unwrap().to_str().unwrap();

    let desired_filename = generate_desired_filename(&metadata, &extenstion);
    let desired_directory_path = generate_desired_directory_path(&metadata);
    let full_desired_path = desired_directory_path.join(desired_filename);

    if full_desired_path == relative_path {
        CheckStatus::Correct
    } else {
        CheckStatus::Wrong
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_generate_desired_filename() {
        // given
        let metadata = PhotoMetadata { datetime: DateTime::from_ascii(b"2019:08:10 18:17:28").unwrap() };

        // when
        let filename = generate_desired_filename(&metadata, "test_photo_ext");

        // then
        assert_eq!(filename, "2019-08-10_18-17-28.test_photo_ext");
    }

    #[test]
    fn it_should_generate_desired_directory_path() {
        // given
        let metadata = PhotoMetadata { datetime: DateTime::from_ascii(b"2019:08:10 18:17:28").unwrap() };

        // when
        let directory_path = generate_desired_directory_path(&metadata);

        // then
        assert_eq!(directory_path, Path::new("").join("2019").join("08").join("10"));
    }

    #[test]
    fn it_checks_correct_location() {
        // given
        let file_path = Path::new("/device/user/Photos/2019/08/10/2019-08-10_18-17-28.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata { datetime: DateTime::from_ascii(b"2019:08:10 18:17:28").unwrap() };

        // when
        let status = check_file(file_path, metadata, root_path);

        // then
        assert_eq!(status, CheckStatus::Correct);
    }

    #[test]
    fn it_checks_wrong_location() {
        // given
        let file_path = Path::new("/device/user/Photos/some_folder/myphoto.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata { datetime: DateTime::from_ascii(b"2019:08:10 18:17:28").unwrap() };

        // when
        let status = check_file(file_path, metadata, root_path);

        // then
        assert_eq!(status, CheckStatus::Wrong);
    }

    #[test]
    fn it_checks_location_with_wrong_filename() {
        // given
        let file_path = Path::new("/device/user/Photos/2019/08/10/myphoto.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata { datetime: DateTime::from_ascii(b"2019:08:10 18:17:28").unwrap() };

        // when
        let status = check_file(file_path, metadata, root_path);

        // then
        assert_eq!(status, CheckStatus::Wrong);
    }

    #[test]
    fn it_checks_location_with_wrong_directory_path() {
        // given
        let file_path = Path::new("/device/user/Photos/some_folder/2019-08-10_18-17-28.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata { datetime: DateTime::from_ascii(b"2019:08:10 18:17:28").unwrap() };

        // when
        let status = check_file(file_path, metadata, root_path);

        // then
        assert_eq!(status, CheckStatus::Wrong);
    }
}
