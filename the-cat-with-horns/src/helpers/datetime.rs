use chrono::{NaiveDate, Datelike};
use ordinal::Ordinal;

pub fn pretty_date(date: &NaiveDate) -> String {
    let day = date.day();
    let day_ordinal = Ordinal(day).to_string();  // could error handle but lazy
    let month = date.format("%B");
    let year = date.year();

    format!("{} {} {}", month, day_ordinal, year)
}
