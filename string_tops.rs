fn tops(s: &str) -> String {
    let mut r = String::new();
    let mut i = 0;
    let mut k = 1;
    let mut x = 0;
    while i<s.len() {
        x += 1;
        if x%2==0 {r += &s[i..i+1]}
        i += k;
        k += 1;
    }
    r.chars().rev().collect()
}