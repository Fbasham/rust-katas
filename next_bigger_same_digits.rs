use itertools::Itertools;

fn next_bigger_number(n: u64) -> Option<u64> {
    let s = n.to_string().chars().collect::<Vec<_>>();
    let x = (1..s.len()).rev().filter(|&i| s[i - 1] < s[i]).next();
    if x == None {
        return None;
    }
    let i = x.unwrap();
    let t = &s[i..s.len()].iter().cloned().sorted().collect::<Vec<_>>();
    let j = (0..t.len()).position(|j| t[j] > s[i - 1]).unwrap();
    Some(
        format!(
            "{}{}{}",
            &s[0..i - 1].iter().join(""),
            t[j],
            (s[i - 1].to_string() + &t[0..j].iter().join("") + &t[j + 1..t.len()].iter().join(""))
                .chars()
                .sorted()
                .join("")
        )
        .parse()
        .unwrap(),
    )
}
