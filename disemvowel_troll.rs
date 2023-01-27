fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|&e| "aeiouAEIOU".contains(e) == false)
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
}
