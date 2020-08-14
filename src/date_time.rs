pub use exif::DateTime; // TODO: exif::DateTime is used only to parse date time in 2020:06:30 20:51:10 format. Parsing should be implemented here and the dependecy dropped entirely.

pub fn datetime_to_iso_string(datetime: &DateTime) -> String {
    format!(
        "{:04}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}Z",
        datetime.year,
        datetime.month,
        datetime.day,
        datetime.hour,
        datetime.minute,
        datetime.second,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_map_datetime_to_iso_string() {
        // given
        let datetime = DateTime::from_ascii(b"2020:06:08 20:02:24").unwrap();

        // when
        let iso_string: String = datetime_to_iso_string(&datetime);

        // then
        assert_eq!(iso_string, "2020-06-08T20:02:24Z");
    }
}
