use itertools::Itertools;
use std::collections::HashMap;

fn by_state(s: &str) -> String {
    let mut d = HashMap::new();
    for e in s.split("\n") {
        let k = match &e[e.len() - 2..] {
            "AZ" => "Arizona",
            "CA" => "California",
            "ID" => "Idaho",
            "IN" => "Indiana",
            "MA" => "Massachusetts",
            "OK" => "Oklahoma",
            "PA" => "Pennsylvania",
            "VA" => "Virginia",
            _ => "",
        };
        if !d.contains_key(&k) {
            d.insert(k, vec![]);
        }
        d.get_mut(k)
            .unwrap()
            .push(format!("{}{}", &e[0..e.len() - 2], k))
    }
    d.keys()
        .sorted()
        .map(|k| {
            format!(
                " {}\n..... {}",
                k,
                d.get(k).unwrap().iter().cloned().sorted().join("\n..... ")
            )
        })
        .join("\n")
        .trim()
        .replace(",", "")
        .to_string()
}
