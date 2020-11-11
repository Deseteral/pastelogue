use std::{error, fmt};

#[derive(Debug)]
pub struct ExifDateTime {
    pub year: String,
    pub month: String,
    pub day: String,
    pub hour: String,
    pub minute: String,
    pub second: String,
}

impl ExifDateTime {
    pub fn from_exif_string(exif_date_str: &str) -> Result<ExifDateTime, ExifDateTimeParsingError> {
        // TODO: Error handling

        // in '2020:06:30 20:51:10' format
        Ok(ExifDateTime {
            year: exif_date_str[0..4].to_owned(),
            month: exif_date_str[5..7].to_owned(),
            day: exif_date_str[8..10].to_owned(),
            hour: exif_date_str[11..13].to_owned(),
            minute: exif_date_str[14..16].to_owned(),
            second: exif_date_str[17..19].to_owned(),
        })
    }

    pub fn to_iso_string(&self) -> String {
        format!(
            "{:04}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}Z",
            self.year, self.month, self.day, self.hour, self.minute, self.second,
        )
    }
}

#[derive(Debug, Clone)]
pub struct ExifDateTimeParsingError;

impl fmt::Display for ExifDateTimeParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse EXIF DateTime")
    }
}

impl error::Error for ExifDateTimeParsingError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl From<std::string::FromUtf8Error> for ExifDateTimeParsingError {
    fn from(_error: std::string::FromUtf8Error) -> Self {
        ExifDateTimeParsingError {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_parse_datetime() {
        // given
        let date_string = "2020:11:11 13:57:24";

        // when
        let datetime: ExifDateTime = ExifDateTime::from_exif_string(date_string).unwrap();

        // then
        assert_eq!(datetime.year, "2020");
        assert_eq!(datetime.month, "11");
        assert_eq!(datetime.day, "11");
        assert_eq!(datetime.hour, "13");
        assert_eq!(datetime.minute, "57");
        assert_eq!(datetime.second, "24");
    }

    #[test]
    fn it_should_map_datetime_to_iso_string() {
        // given
        let datetime = ExifDateTime::from_exif_string("2020:06:08 20:02:24").unwrap();

        // when
        let iso_string: String = datetime.to_iso_string();

        // then
        assert_eq!(iso_string, "2020-06-08T20:02:24Z");
    }
}
