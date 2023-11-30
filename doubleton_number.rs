use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    static ref A: Vec<u32> = {
        let mut v = vec![];
        let mut n = 10;
        while n < 1000000 {
            if n.to_string().chars().unique().count() == 2 {
                v.push(n);
            }
            n += 1;
        }
        v
    };
}

fn doubleton(n: u32) -> u32 {
    let i = A.iter().position(|&e| e >= n).unwrap();
    if A[i] == n {
        A[i + 1]
    } else {
        A[i]
    }
}
