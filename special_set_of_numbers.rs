fn find_us(n1: u32, n2: u32, k: u32, pf: &[u32], d: &[u32]) -> Vec<u32> {
    let mut v = vec![];
    for i in n1..=n1 + k * n2 {
        if pf.iter().all(|e| i % e == 0)
            && d.iter()
                .all(|e| i.to_string().contains(e.to_string().as_str()))
        {
            v.push(i);
        }
    }
    v
}
