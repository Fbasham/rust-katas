use chrono::{NaiveDate,Days};

fn life_predictor(date: &str) -> String {
    NaiveDate::parse_from_str(date,"%Y-%m-%d").unwrap().checked_sub_days(Days::new(280)).unwrap().to_string()
}