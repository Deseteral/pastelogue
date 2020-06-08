pub use exif::DateTime as DateTime;

pub fn datetime_to_iso_string(datetime: &DateTime) -> String {
    format!(
        "{:04}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}",
        datetime.year,
        datetime.month,
        datetime.day,
        datetime.hour,
        datetime.minute,
        datetime.second,
    )
}
