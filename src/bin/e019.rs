use chrono::{Datelike, NaiveDate};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    (1901..=2000)
        .map(|y| {
            (1..=12)
                .map(|m| NaiveDate::from_ymd_opt(y, m, 1).unwrap().weekday())
                .filter(|d| *d == chrono::Weekday::Sun)
                .count()
        })
        .sum()
}
