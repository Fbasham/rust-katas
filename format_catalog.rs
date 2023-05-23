use itertools::Itertools;
use regex::Regex;

fn catalog(s: &str, item: &str) -> String {
    let r = s
        .split("\n\n")
        .filter(|&t| {
            Regex::new(format!("<name>.*{}.*</name>", item).as_str())
                .unwrap()
                .is_match(t)
        })
        .map(|e| {
            let n = Regex::new(r"<name>.*?</name>")
                .unwrap()
                .find(e)
                .unwrap()
                .as_str();
            let t = Regex::new(r"<(prx|qty)>(.*?)</(prx|qty)>")
                .unwrap()
                .find_iter(e)
                .map(|m| &m.as_str()[5..m.as_str().len() - 6])
                .collect::<Vec<_>>();
            return format!("{} > prx: ${} qty: {}", &n[6..n.len() - 7], t[0], t[1]);
        })
        .join("\n");
    if r == "" {
        "Nothing".to_string()
    } else {
        r
    }
}
