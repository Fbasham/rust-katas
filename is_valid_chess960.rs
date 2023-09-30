use regex::Regex;

fn is_valid(s: &str) -> bool {
    Regex::new(r"R.*K.*R").unwrap().is_match(s) && 
    Regex::new(r"B(..)*B").unwrap().is_match(s) && 
    !Regex::new(r"^[^R]*K.*R").unwrap().is_match(s) &&
    s.len()==8
}