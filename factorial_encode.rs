fn f(n: usize) -> u64 {
    (1..=n).product::<usize>() as u64
}

fn dec2_fact_string(n: u64) -> String {
    let a = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let d = (0..).find(|&i| f(i) > n).unwrap() - 1;
    let mut t = n;
    let mut s = String::new();
    for i in (0..=d).rev() {
        let x = t / f(i);
        t %= f(i);
        s.push(a.chars().nth(x as usize).unwrap());
    }
    s
}

fn fact_string_2dec(s: String) -> u64 {
    let t = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, e)| t.chars().position(|x| x == e).unwrap() as u64 * f(i))
        .sum()
}
