use chrono::*;
use itertools::Itertools;
use std::collections::HashMap;

fn most_frequent_days(year: i32) -> Vec<String> {
    let mut d = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    let mut h = HashMap::new();
    let a = vec![
        Weekday::Mon,
        Weekday::Tue,
        Weekday::Wed,
        Weekday::Thu,
        Weekday::Fri,
        Weekday::Sat,
        Weekday::Sun,
    ];
    while d.year() == year {
        let k = d.weekday();
        if !h.contains_key(&k) {
            h.insert(k, 0);
        }
        h.insert(k, h.get(&k).unwrap() + 1);
        d += Duration::days(1);
    }
    let m = h.values().max().unwrap();
    h.keys()
        .filter(|k| h.get(&k).unwrap() == m)
        .sorted_by_key(|&k| a.iter().position(|e| e == k))
        .map(|k| match k {
            Weekday::Mon => "Monday".to_string(),
            Weekday::Tue => "Tuesday".to_string(),
            Weekday::Wed => "Wednesday".to_string(),
            Weekday::Thu => "Thursday".to_string(),
            Weekday::Fri => "Friday".to_string(),
            Weekday::Sat => "Saturday".to_string(),
            Weekday::Sun => "Sunday".to_string(),
        })
        .collect::<Vec<_>>()
}
