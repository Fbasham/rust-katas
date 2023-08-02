use std::collections::HashMap;

fn count_contiguous_distinct(k: u32, a: &[i32]) -> Vec<u32> {
    let mut d = HashMap::new();
    let mut r = vec![];
    for e in &a[..k as usize] {
        d.insert(e,d.get(e).unwrap_or(&0)+1);
    }
    r.push(d.len() as u32);
    for i in 1..a.len()-(k as usize)+1 {
        d.insert(&a[i-1],d.get(&a[i-1]).unwrap()-1);
        if d.get(&a[i-1]).unwrap()==&0 {d.remove(&a[i-1]);}
        d.insert(&a[i+(k as usize)-1],d.get(&a[i+(k as usize)-1]).unwrap_or(&0)+1);
        r.push(d.len() as u32)
    }
    r
}