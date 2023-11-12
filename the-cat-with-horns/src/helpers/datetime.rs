use poise::serenity_prelude::Timestamp;
use ordinal::Ordinal;

pub fn pretty_date(ts: &Timestamp) -> String {
    let day = ts.format("%d").to_string().parse::<i32>();
    let day_ordinal = Ordinal(day.unwrap_or(0)).to_string();  // could error handle but lazy
    let month = ts.format("%B");
    let year = ts.format("%Y");

    format!("{} {} {}", month, day_ordinal, year)
}