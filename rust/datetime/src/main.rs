use chrono::{DateTime, Timelike};

fn main() {
    let text = "2024/07/31 15:00 +09:00";
    let dt = DateTime::parse_from_str(text, "%Y/%m/%d %H:%M %z").unwrap();
    println!("{:?}", dt.timestamp_millis());
}
