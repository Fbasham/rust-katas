use std::collections::HashSet;

fn integer_triangles(p: u32) -> u32 {
    let mut r = HashSet::new();
    for b in 1..p {
        for c in 1..p {
            let a = ((b * b + c * c + b * c) as f64).sqrt() as u32;
            if a + b + c > p {
                break;
            }
            if a * a == b * b + c * c + b * c {
                let mut v = vec![a, b, c];
                v.sort();
                r.insert((v[0], v[1], v[2]));
            }
        }
    }
    r.len() as u32
}
