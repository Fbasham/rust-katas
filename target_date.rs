use chrono::*;

fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
    let n = ((a as f64 / a0 as f64).ln() / (1.0 + (p as f64) / 36000.0).ln()).ceil();
    let d = NaiveDate::from_ymd_opt(2016, 1, 1).unwrap();
    (d + Days::new(n as u64)).to_string()
}
