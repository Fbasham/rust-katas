use std::collections::HashSet;

fn f(n: u32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    n > 1
}

fn solve(a: u32, b: u32) -> usize {
    let mut c = 0;
    for i in a..b {
        if f(i) {
            let mut s = HashSet::new();
            let mut j = i;
            while j != 1 && !s.contains(&j) {
                s.insert(j);
                j = j
                    .to_string()
                    .chars()
                    .fold(0, |a, x| a + ((x as u32) - 48).pow(2));
            }
            if j == 1 {
                c += 1;
            }
        }
    }
    c
}
