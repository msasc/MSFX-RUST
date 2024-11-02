use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc, Local, TimeZone};

fn main() {

    // 1. Get the current date and time in UTC
    let now_utc: DateTime<Utc> = Utc::now();
    println!("Current UTC date and time: {}", now_utc);

    // 2. Get the current date and time in the local timezone
    let now_local: DateTime<Local> = Local::now();
    println!("Current local date and time: {}", now_local);

    // 3. Create a specific date using `from_ymd_opt`
    let specific_date = NaiveDate::from_ymd_opt(2024, 12, 31)
        .expect("Invalid date"); // Handle the Option
    println!("Specific date: {}", specific_date);

    // 4. Create a specific time using `from_hms_opt`
    let specific_time = NaiveTime::from_hms_opt(23, 59, 59)
        .expect("Invalid time"); // Handle the Option
    println!("Specific time: {}", specific_time);

    // 5. Create a specific datetime (naive - without timezone)
    let specific_datetime = NaiveDateTime::new(specific_date, specific_time);
    println!("Specific datetime: {}", specific_datetime);

    // 6. Parse a date and time from a string
    let parsed_datetime = "2024-12-31T23:59:59"
        .parse::<NaiveDateTime>()
        .expect("Failed to parse date and time");
    println!("Parsed datetime: {}", parsed_datetime);

    // 7. Format a datetime as a string
    let formatted = now_local.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Formatted current local datetime: {}", formatted);

    // 8. Convert a NaiveDateTime to a DateTime with timezone (e.g., UTC)
    let utc_datetime: DateTime<Utc> = Utc.from_utc_datetime(&specific_datetime);
    println!("Specific datetime with UTC timezone: {}", utc_datetime);

    // 9. Calculate the difference between two dates
    let duration = now_utc.signed_duration_since(utc_datetime);
    println!("Duration since specific datetime: {} seconds", duration.num_seconds());
}
