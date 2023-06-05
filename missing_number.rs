use itertools::Itertools;

fn find_number(a: u32, b: u32, s: &str) -> Vec<u32> {
    let d = (a..=b).map(|e| e.to_string()).join("").chars().counts();
    let v = s.chars().counts();
    let mut r = vec![];
    let mut m = 0;
    for i in a..=b {
        if i.to_string().chars().all(|e| {
            let n = i.to_string().chars().filter(|&x| e == x).count();
            v.get(&e).unwrap_or(&0) + n == *d.get(&e).unwrap_or(&0)
        }) {
            r.push(i);
            m = m.max(i.to_string().len());
        }
    }
    r.iter()
        .filter(|e| e.to_string().len() == m)
        .cloned()
        .collect()
}
