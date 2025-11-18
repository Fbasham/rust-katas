fn first_non_repeating(s: &str) -> Option<char> {
    s.chars().find(|e| s.to_lowercase().chars().filter(|&x| x==e.to_ascii_lowercase()).count()==1)
}