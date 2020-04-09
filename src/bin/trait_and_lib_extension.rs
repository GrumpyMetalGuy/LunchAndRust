use chrono::{Datelike, NaiveDate, Weekday};

trait ExtraDateFunctionality {
    fn is_weekend(&self) -> bool;
}

impl ExtraDateFunctionality for NaiveDate {
    fn is_weekend(&self) -> bool {
        match self.weekday() {
            Weekday::Sat | Weekday::Sun => true,
            _ => false,
        }
    }
}

fn main() {
    let saturday = NaiveDate::from_ymd(2020, 4, 4);
    let sunday = NaiveDate::from_ymd(2020, 4, 5);
    let monday = NaiveDate::from_ymd(2020, 4, 6);

    println!("Saturday is on the weekend: {}", saturday.is_weekend());
    println!("Sunday is on the weekend: {}", sunday.is_weekend());
    println!("Monday is on the weekend: {}", monday.is_weekend());
}
