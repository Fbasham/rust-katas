use itertools::Itertools;

fn find_all(n: u8, d: u8) -> Option<(usize, u64, u64)> {
    let mut k = 0;
    let mut x = u64::MAX;
    let mut y = 0;
    for c in (1..10).combinations_with_replacement(d as usize) {
        let s = c.iter().map(|e| e.to_string()).join("").parse::<u64>().unwrap();
        if c.iter().sum::<u8>()==n && c.iter().zip(&c[1..]).all(|(i,j)| j>=i) {
            k += 1;
            x = s.min(x);
            y = s.max(y);
        }
    }
    match y==0 {
        false => Some((k,x,y)),
        _ => None
    }
}