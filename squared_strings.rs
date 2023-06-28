use itertools::Itertools;

fn code(msg: &str) -> String {
    let n = (msg.len() as f64).sqrt().ceil() as usize;
    let s = msg.to_owned() + &(0..n * n - msg.len()).map(|_| '\x0b').collect::<String>();
    let mut v = vec![];
    for i in (0..n * n).step_by(n) {
        v.push(&s[i..i + n]);
    }
    (0..n)
        .map(|i| {
            v.iter()
                .map(|t| t.chars().nth(i).unwrap())
                .rev()
                .collect::<String>()
        })
        .join("\n")
}

fn decode(s: &str) -> String {
    let mut v = s.split("\n").collect::<Vec<_>>();
    (0..v[0].len())
        .map(|i| {
            v.iter()
                .map(|t| t.chars().nth(v.len() - i - 1).unwrap())
                .filter(|&e| e != '\x0b')
                .collect::<String>()
        })
        .join("")
}
