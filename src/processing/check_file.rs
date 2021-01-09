use crate::exif::extract_metadata::PhotoMetadata;
use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug)]
pub enum FilePathCheckStatus {
    Correct,
    Wrong(PathBuf),
}

pub fn check_file_path_with_metadata(
    file_path: &Path,
    metadata: &PhotoMetadata,
    root_path: &Path,
) -> FilePathCheckStatus {
    let relative_path = file_path.strip_prefix(root_path).unwrap();
    let extenstion = file_path.extension().unwrap().to_str().unwrap(); // TODO: This should not convert to &str, use &OsStr instead

    let desired_filename = generate_desired_filename(&metadata, &extenstion);
    let desired_directory_path = generate_desired_directory_path(&metadata);
    let relative_desired_path = desired_directory_path.join(desired_filename);

    if relative_desired_path == relative_path {
        FilePathCheckStatus::Correct
    } else {
        let full_desired_path = root_path.join(relative_desired_path);
        FilePathCheckStatus::Wrong(full_desired_path)
    }
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
        extension.to_lowercase(),
    )
}

fn generate_desired_directory_path(metadata: &PhotoMetadata) -> PathBuf {
    Path::new("")
        .join(format!("{:04}", metadata.datetime.year))
        .join(format!("{:0>2}", metadata.datetime.month))
        .join(format!("{:0>2}", metadata.datetime.day))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exif::exif_date_time::ExifDateTime;

    #[test]
    fn it_should_generate_desired_filename() {
        // given
        let metadata = PhotoMetadata {
            datetime: ExifDateTime::from_exif_string("2019:08:10 18:17:28").unwrap(),
        };

        // when
        let filename = generate_desired_filename(&metadata, "TEST_PHOTO_EXT");

        // then
        assert_eq!(filename, "2019-08-10_18-17-28.test_photo_ext");
    }

    #[test]
    fn it_should_generate_desired_directory_path() {
        // given
        let metadata = PhotoMetadata {
            datetime: ExifDateTime::from_exif_string("2019:08:10 18:17:28").unwrap(),
        };

        // when
        let directory_path = generate_desired_directory_path(&metadata);

        // then
        assert_eq!(
            directory_path,
            Path::new("").join("2019").join("08").join("10")
        );
    }

    #[test]
    fn it_checks_correct_location() {
        // given
        let file_path = Path::new("/device/user/Photos/2019/08/10/2019-08-10_18-17-28.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata {
            datetime: ExifDateTime::from_exif_string("2019:08:10 18:17:28").unwrap(),
        };

        // when
        let status = check_file_path_with_metadata(&file_path, &metadata, &root_path);

        // then
        assert_eq!(status, FilePathCheckStatus::Correct);
    }

    #[test]
    fn it_checks_wrong_location() {
        // given
        let file_path = Path::new("/device/user/Photos/some_folder/myphoto.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata {
            datetime: ExifDateTime::from_exif_string("2019:08:10 18:17:28").unwrap(),
        };

        // when
        let status = check_file_path_with_metadata(&file_path, &metadata, &root_path);

        // then
        match status {
            FilePathCheckStatus::Wrong(correct_path) => assert_eq!(
                correct_path,
                PathBuf::from("/device/user/Photos/2019/08/10/2019-08-10_18-17-28.jpg")
            ),
            _ => panic!("Wrong status type"),
        };
    }

    #[test]
    fn it_checks_location_with_wrong_filename() {
        // given
        let file_path = Path::new("/device/user/Photos/2019/08/10/myphoto.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata {
            datetime: ExifDateTime::from_exif_string("2019:08:10 18:17:28").unwrap(),
        };

        // when
        let status = check_file_path_with_metadata(&file_path, &metadata, &root_path);

        // then
        match status {
            FilePathCheckStatus::Wrong(correct_path) => assert_eq!(
                correct_path,
                PathBuf::from("/device/user/Photos/2019/08/10/2019-08-10_18-17-28.jpg")
            ),
            _ => panic!("Wrong status type"),
        };
    }

    #[test]
    fn it_checks_location_with_wrong_directory_path() {
        // given
        let file_path = Path::new("/device/user/Photos/some_folder/2019-08-10_18-17-28.jpg");
        let root_path = Path::new("/device/user/Photos");
        let metadata = PhotoMetadata {
            datetime: ExifDateTime::from_exif_string("2019:08:10 18:17:28").unwrap(),
        };

        // when
        let status = check_file_path_with_metadata(&file_path, &metadata, &root_path);

        // then
        match status {
            FilePathCheckStatus::Wrong(correct_path) => assert_eq!(
                correct_path,
                PathBuf::from("/device/user/Photos/2019/08/10/2019-08-10_18-17-28.jpg")
            ),
            _ => panic!("Wrong status type"),
        };
    }
}
