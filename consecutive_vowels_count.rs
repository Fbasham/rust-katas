fn get_the_vowels(s: &str) -> u32 {
    let mut c = 0;
    let mut v = "aeiou";
    let mut i = 0;
    for e in s.chars() {
        if e == v.chars().nth(i).unwrap() {
            c += 1;
            i = (i + 1) % 5;
        }
    }
    c
}
