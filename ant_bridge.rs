fn ant_bridge(ants: &str, terrain: &str) -> String {
    let s = (terrain.to_owned() + &"-".repeat(ants.len()))
        .chars()
        .collect::<Vec<_>>();
    let mut x = ants.to_string();
    for i in 0..s.len() - 1 {
        if s[i] == '.'
            || (s[i] == '-' && s[i + 1] == '.')
            || (i > 0 && s[i] == '-' && s[i - 1] == '.')
        {
            x = format!("{}{}", &x[x.len() - 1..], &x[..x.len() - 1]);
        }
    }
    x
}
