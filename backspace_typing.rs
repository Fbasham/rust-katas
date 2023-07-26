use regex::Regex;

fn solve(s: &str) -> String {
    let mut v = vec![];
    for m in Regex::new(r"\[backspace\]\*\d*|\[backspace\]|.").unwrap().find_iter(s) {
        let e = m.as_str();
        if e.contains("[backspace]") {
            let x = if e.contains("*") {e.split("*").last().unwrap().parse().unwrap()} else {1};
            for _ in 0..x {
                if v.len()>0 {
                    v.pop();
                }
            }
        }
        else {
            v.push(e)
        }
    }
    v.join("")
}