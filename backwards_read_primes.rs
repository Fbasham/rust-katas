fn is_prime(n: u64) -> bool {
    if n==2 {return true};
    if n<2 || n%2==0 {return false};
    let mut i = 2;
    while i*i<=n {
        if n%i==0 {return false;}
        i += 1;
    }
    return true;
}

fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut v = vec![];
    for i in start..=stop {
        let j = i.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
        if !v.contains(&i) && is_prime(i) && is_prime(j) && i != j {
            v.push(i);
            if j>=start && j<=stop {v.push(j);}
        }
    }
    v.sort();
    v
}