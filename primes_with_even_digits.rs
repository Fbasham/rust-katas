use lazy_static::lazy_static;

// global variable declaration at runtime
lazy_static! {
    static ref v: Vec<u32> = (3..5000000)
        .filter(|&n| {
            if n % 2 == 0 {
                return n == 2;
            }
            for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<u32>>();
}

fn f(n: u32) -> u32 {
    *v.iter()
        .filter(|&e| e < &n)
        .max_by_key(|e| {
            e.to_string()
                .chars()
                .filter(|&x| (x as u8 - 48) & 1 == 0)
                .count()
        })
        .unwrap()
}
