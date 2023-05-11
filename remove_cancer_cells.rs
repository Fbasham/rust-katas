use regex::Regex;

fn cut_cancer_cells(s: &str) -> String {
    Regex::new(r"[^A-Z]?C[^A-Z]?|c")
        .unwrap()
        .replace_all(s, "")
        .to_string()
}
