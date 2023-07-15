use lazy_static::lazy_static;

lazy_static! {
    static ref P: Vec<u32> = vec![
        7, 97, 16057, 19417, 43777, 1091257, 1615837, 1954357, 2822707, 2839927, 3243337, 3400207,
        6005887, 6503587, 7187767, 7641367, 8061997, 8741137
    ];
}

fn find_primes_sextuplet(n: u32) -> Vec<u32> {
    println!("{}", n);
    for i in P.iter().cloned() {
        let v = vec![i, i + 4, i + 6, i + 10, i + 12, i + 16];
        if v.iter().sum::<u32>() > n {
            return v;
        }
    }
    vec![]
}
