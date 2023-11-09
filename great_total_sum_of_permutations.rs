use itertools::Itertools;

fn gta(k: u32, a: &[u32]) -> u32 {
    let mut v = a.iter().map(|e| e.to_string()).collect::<Vec<String>>();
    let mut d = vec![];
    let mut i = 0;
    while v.iter().any(|e| e.len() > 0) {
        let j = i % v.len();
        if v[j].len() > 0 {
            let e = v[j][..1].parse::<u32>().unwrap();
            v[j] = format!("{}", &v[j][1..]);
            if !d.contains(&e) {
                d.push(e);
            }
        }
        i += 1
    }
    (1..=k as usize)
        .map(|i| {
            d[..k as usize]
                .iter()
                .permutations(i)
                .map(|p| p.iter().cloned().sum::<u32>())
                .sum::<u32>()
        })
        .sum()
}
