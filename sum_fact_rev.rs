use std::collections::HashSet;

fn f(mut n: u32) -> HashSet<u32> {
    let mut s = HashSet::new();
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            s.insert(i);
            n = n / i;
        }
        i += 1;
    }
    if n > 1 {
        s.insert(n);
    }
    s
}

fn same_fact_rev(n: u32) -> Vec<u32> {
    let mut v = vec![];
    for i in 1089..n {
        let j = i
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();
        if f(i) == f(j) && i != j {
            v.push(i);
        }
    }
    v
}
