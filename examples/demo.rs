use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use datetime_rs::prelude::*;

fn main() {
    let friday = next_weekday(1, &Weekday::Fri);
    println!("Next Friday: {}", friday);
    println!("Next Friday: {}", friday.format_default());
    
    println!("Next Friday: {}", friday.timestamp());
    // Next Friday at midnight: 1686844800
}
