use itertools::Itertools;
use regex::Regex;

fn roadkill(photo: &str) -> String {
    let s = photo.replace("=", "");
    preloaded::ANIMALS
        .iter()
        .find(|a| {
            let re = Regex::new(format!("^{}+$", a.chars().join("+")).as_str()).unwrap();
            re.is_match(s.as_str()) || re.is_match(s.chars().rev().join("").as_str())
        })
        .unwrap_or(&"??")
        .to_string()
}
