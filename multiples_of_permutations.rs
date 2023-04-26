use itertools::Itertools;

fn search_perm_mult(n: u32, k: u32) -> u32 {
    let mut c = 0;
    for i in 1..n {
        let s = i.to_string().chars().sorted().collect::<Vec<_>>();
        let t = (i * k).to_string().chars().sorted().collect::<Vec<_>>();
        if s == t && i * k < n {
            c += 1;
        }
    }
    c
}
