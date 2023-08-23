use itertools::Itertools;

fn is_prime(n: u32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    for i in (3..((n as f64).sqrt() as u32 + 1)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn special_primes(n: u32) -> [(u32, u32, u32); 3] {
    let mut a = (u32::MAX, 0, 0);
    let mut b = (u32::MAX, 0, 0);
    let mut c = (u32::MAX, 0, 0);
    for i in 100..=n {
        let s = i.to_string();
        let t = s.chars().map(|e| e.to_digit(10).unwrap()).sum::<u32>();
        let f = (2..(t as f64).sqrt() as u32 + 1).any(|j| t % (j * j) == 0);
        let g = s.chars().nth(0).unwrap().to_digit(10).unwrap()
            * s.chars().last().unwrap().to_digit(10).unwrap()
            != 45;
        if is_prime(i) && !s.contains("0") && f && g && s.chars().unique().count() == s.len() {
            let d = s
                .chars()
                .zip(s.chars().skip(1))
                .map(|(i, j)| (j as i8 - i as i8).signum())
                .collect::<Vec<_>>();
            if d.iter().unique().count() == 1 && d[0] == 1 {
                b = (i.min(b.0), i.max(b.1), b.2 + 1);
            } else if d.iter().unique().count() == 1 && d[0] == -1 {
                c = (i.min(c.0), i.max(c.1), c.2 + 1);
            } else {
                a = (i.min(a.0), i.max(a.1), a.2 + 1);
            }
        }
    }
    a.0 = if a.0 == u32::MAX { 0 } else { a.0 };
    b.0 = if b.0 == u32::MAX { 0 } else { b.0 };
    c.0 = if c.0 == u32::MAX { 0 } else { c.0 };
    [a, b, c]
}
