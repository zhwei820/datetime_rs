# rust datetime util

```rs

use chrono::Weekday;
use datetime_rs::prelude::*;

fn main() {
    let friday = next_weekday(1, &Weekday::Fri);
    println!("Next Friday: {}", friday);
    // Next Friday: 2023-06-16 00:00:00.068642 +08:00
    println!("Next Friday: {}", friday.format_default());
    // Next Friday: 2023-06-16 00:00:00
    println!("Next Friday: {}", friday.timestamp());
    // Next Friday at midnight: 1686844800
}

```
