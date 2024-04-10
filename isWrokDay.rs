use chrono::{NaiveDate, Weekday, Datelike};
use std::collections::HashSet;

static mut LOCALE: &str = "en-US"; // Default locale
static mut HOLIDAYS: HashSet<String> = HashSet::new(); // Default holidays list

fn is_workday(date: &str) -> bool {
    let dt = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();

    // Check if it's a weekend
    if dt.weekday().number_from_monday() > 5 {
        return false;
    }

    // Check if it's a holiday
    let formatted_date = dt.format("%Y-%m-%d").to_string();
    unsafe { !HOLIDAYS.contains(&formatted_date) }
}

fn update(new_locale: &str, new_holidays: Vec<&str>) {
    unsafe {
        LOCALE = new_locale;
        HOLIDAYS.extend(new_holidays.iter().map(|&s| s.to_string()));
    }
}

fn get_next_workday(date: &str) -> String {
    let mut dt = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap() + chrono::Duration::days(1);
    while !is_workday(&dt.format("%Y-%m-%d").to_string()) {
        dt += chrono::Duration::days(1);
    }
    dt.format("%Y-%m-%d").to_string()
}

fn is_holiday(date: &str) -> bool {
    let dt = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();
    let formatted_date = dt.format("%Y-%m-%d").to_string();
    unsafe { HOLIDAYS.contains(&formatted_date) }
}

fn add_holiday(date: &str) {
    unsafe {
        HOLIDAYS.insert(date.to_string());
    }
}

fn remove_holiday(date: &str) {
    unsafe {
        HOLIDAYS.remove(date);
    }
}

fn get_workdays_in_range(start_date: &str, end_date: &str) -> Vec<String> {
    let mut workdays = Vec::new();
    let mut current_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").unwrap();
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").unwrap();

    while current_date <= end {
        if is_workday(&current_date.format("%Y-%m-%d").to_string()) {
            workdays.push(current_date.format("%Y-%m-%d").to_string());
        }
        current_date += chrono::Duration::days(1);
    }

    workdays
}

fn get_holiday_name(date: &str) -> Option<String> {
    let formatted_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap().format("%Y-%m-%d").to_string();
    unsafe { HOLIDAYS.get(&formatted_date).cloned() }
}

// Example usage:
// update("en-US", vec!["2024-01-01", "2024-12-25"]);
// println!("{}", is_workday("2024-04-15"));
// println!("{}", get_next_workday("2024-04-15"));
// println!("{}", is_holiday("2024-04-15"));
// add_holiday("2024-05-01");
// remove_holiday("2024-12-25");
// println!("{:?}", get_workdays_in_range("2024-04-01", "2024-04-30"));
// println!("{:?}", get_holiday_name("2024-04-15"));
