use itertools::Itertools;

fn next_smaller_number(n: u64) -> Option<u64> {
    let s = n.to_string().chars().collect::<Vec<_>>();
    let mut i = s.len() - 1;
    while i > 0 {
        if s[i - 1] > s[i] {
            break;
        }
        i -= 1;
    }
    if i == 0 {
        return None;
    }
    let t = &s[i - 1..];
    let x = t
        .iter()
        .cloned()
        .sorted()
        .rev()
        .filter(|e| e < &s[i - 1])
        .next()
        .unwrap();
    let y = (t.iter().join("") + s[i].to_string().as_str())
        .chars()
        .sorted()
        .rev()
        .join("")
        .replacen(x, "", 1);
    let tt = format!("{}{}{}", s[..i - 1].iter().join(""), x, &y[..y.len() - 1]);
    let r = tt.parse::<u64>().unwrap();
    if r.to_string().len() == s.len() {
        Some(r)
    } else {
        None
    }
}
