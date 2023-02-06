use regex::Regex;

fn solve(s: &str) -> u32 {
    Regex::new(r"\d+")
        .unwrap()
        .find_iter(s)
        .map(|e| e.as_str().parse::<u32>().unwrap())
        .max()
        .unwrap_or(0)
}
