fn f(n: u64) -> Vec<u64> {
    let mut v: Vec<u64> = vec![];
    for i in n..100000 {
        v.push(if v.is_empty() {
            i
        } else {
            v[v.len() - 1]
                + v[v.len() - 1]
                    .to_string()
                    .chars()
                    .fold(1, |a, c| a * ((c as u64) - 48).max(1))
        })
    }
    v
}

fn convergence(n: u32) -> usize {
    let b = f(1 as u64);
    f(n as u64).iter().position(|e| b.contains(e)).unwrap()
}
