use itertools::Itertools;

fn nico(key: &str, msg: &str) -> String {
    let t = key
        .chars()
        .map(|e| key.chars().sorted().position(|x| e == x).unwrap() + 1)
        .collect::<Vec<_>>();
    let s = (msg.to_string()
        + &" "
            .repeat((msg.len() as f64 / key.len() as f64).ceil() as usize * key.len() - msg.len()))
        .chars()
        .collect::<Vec<_>>();
    let mut r = String::new();
    for c in s.chunks(key.len()) {
        r += &c
            .iter()
            .zip(t.iter().cloned())
            .sorted_by_key(|k| k.1)
            .map(|t| t.0)
            .join("");
    }
    r
}
