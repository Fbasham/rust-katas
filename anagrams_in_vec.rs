use itertools::Itertools;

fn find_anagrams(a: Vec<String>, s: String) -> Vec<String> {
    let t = s.chars().sorted().join("");
    a.iter()
        .cloned()
        .filter(|e| e.chars().filter(|e| e.is_alphabetic()).sorted().join("") == t)
        .collect()
}
