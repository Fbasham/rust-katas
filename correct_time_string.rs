use regex::Regex;

fn time_correct(t: &str) -> Option<String> {
    if !Regex::new(r"\d\d:\d\d:\d\d").unwrap().is_match(t) {return None}
    let mut v = t.split(":").map(|e| e.parse::<u8>().unwrap()).collect::<Vec<_>>();
    let (mut h, mut m, mut s) = (v[0],v[1],v[2]);
    m = m+(if s>59 {1} else {0});
    s = s%60;
    h = h+(if m>59 {1} else {0});
    m %= 60;
    h %= 24;
    Some(format!("{h:02}:{m:02}:{s:02}"))
}