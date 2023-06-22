fn interpreter(tape: &str, data: &str) -> String {
    let t = tape.chars().collect::<Vec<char>>();
    let mut v = data.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut p = 0;
    while p < v.len() {
        let e = t[i % t.len()];
        i += 1;
        match e == '1' {
            true => v[p] = if v[p] == '0' { '1' } else { '0' },
            _ => p += 1,
        }
    }
    v.iter().collect()
}
