fn my_first_interpreter(s: &str) -> String {
    let mut r = String::new();
    let mut m = 0 as u8;
    for e in s.chars() {
        if e=='+' {m = if m==255 {0} else {m+1}}
        if e=='.' {r += &(m as char).to_string()}
    }
    r
}