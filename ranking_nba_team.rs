use regex::Regex;

fn nba_cup(s: &str, team: &str) -> String {
    let mut w = 0;
    let mut l = 0;
    let mut d = 0;
    let mut p = 0;
    let mut c = 0;
    let mut v = 0;
    let re = Regex::new(r"([a-zA-Z]+|\d+[a-z]+| )+\d+\.?\d*").unwrap();
    for e in s.split(",") {
        let m = re.find_iter(e).map(|e| e.as_str()).collect::<Vec<_>>();
        let (t1, st1) = m[0].trim().rsplit_once(" ").unwrap();
        let (t2, st2) = m[1].trim().rsplit_once(" ").unwrap();
        if t1 == team || t2 == team {
            let s1 = st1.parse::<i32>().unwrap_or(-1);
            let s2 = st2.parse::<i32>().unwrap_or(-1);
            if s1 == -1 || s2 == -1 {
                return format!("Error(float number):{}", e);
            }
            if t1 == team {
                w = w + (if s1 > s2 { 1 } else { 0 });
                l = l + (if s1 < s2 { 1 } else { 0 });
                d = d + (if s1 == s2 { 1 } else { 0 });
                p = p + s1;
                c = c + s2;
                v = v
                    + (if s1 > s2 {
                        3
                    } else if s1 == s2 {
                        1
                    } else {
                        0
                    });
            } else {
                w = w + (if s2 > s1 { 1 } else { 0 });
                l = l + (if s2 < s1 { 1 } else { 0 });
                d = d + (if s2 == s1 { 1 } else { 0 });
                p = p + s2;
                c = c + s1;
                v = v
                    + (if s2 > s1 {
                        3
                    } else if s2 == s1 {
                        1
                    } else {
                        0
                    });
            }
        }
    }
    match (w, l, d) {
        (0, 0, 0) if team == "" => String::new(),
        (0, 0, 0) if team != "" => format!("{}:This team didn't play!", team),
        _ => format!(
            "{}:W={};D={};L={};Scored={};Conceded={};Points={}",
            team, w, d, l, p, c, v
        ),
    }
}
