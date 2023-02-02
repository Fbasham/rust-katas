fn max_rot(n: u64) -> u64 {
    if n < 10 {
        return n;
    }
    let mut s = n.to_string();
    s = format!("{}{}{}", &s[1..2], &s[2..], &s[..1]);
    let mut m = s.parse::<u64>().unwrap();
    for i in 1..s.len() - 1 {
        s = format!("{}{}{}", &s[..i], &s[i + 1..], &s[i..i + 1]);
        m = m.max(s.parse::<u64>().unwrap());
    }
    n.max(m)
}
