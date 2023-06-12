use itertools::Itertools;

fn next_perfectsq_perm(l: u32, k: u32) -> u32 {
    for i in (l as f64).sqrt().ceil() as u32..=3000 {
        let s = (i * i).to_string();
        if !s.contains("0") {
            let mut v = vec![];
            for p in s.chars().permutations(s.len()) {
                let n = p.iter().join("").parse::<f64>().unwrap();
                if n.sqrt() % 1.0 == 0.0 && !v.contains(&(n as u32)) {
                    v.push(n as u32);
                }
            }
            if v.len() as u32 == k {
                return v.iter().cloned().max().unwrap();
            }
        }
    }
    0
}
