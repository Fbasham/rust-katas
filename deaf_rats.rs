use regex::Regex;

fn f(s: &str, t: &str) -> u8 {
    Regex::new("..").unwrap().find_iter(&s.replace(" ","")).filter(|&e| e.as_str()==t).count() as u8
}

fn count_deaf_rats(s: &str) -> u8 {
    f(s.split("P").nth(0).unwrap(),"O~")+f(s.split("P").nth(1).unwrap(),"~O")
}