use chrono::{Days, Months};

pub fn add_months(date: chrono::NaiveDate, months: u32) -> chrono::NaiveDate {
    let mut due_date = date;
    for _ in 0..months {
        due_date = due_date.checked_add_months(Months::new(1)).unwrap();
    }
    return due_date;
}

pub fn add_days(date: chrono::NaiveDate, days: i64) -> chrono::NaiveDate {
    let mut due_date = date;
    for _ in 0..days {
        due_date = due_date.checked_add_days(Days::new(1)).unwrap();
    }
    return due_date;
}
