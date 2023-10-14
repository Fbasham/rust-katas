use std::collections::HashMap;

fn binarray(a: &[u8]) -> u32 {
    let n = a.len();
    let mut d = HashMap::new();
    let mut s = 0;
    let mut m = 0;
    let mut v = a.iter().map(|e| if e==&0 {-1} else {1}).collect::<Vec<_>>();
    for i in 0..v.len() {
        s += v[i];
        if s==0 {m = i+1}
        if d.contains_key(&s){
            if m<i-d.get(&s).unwrap() {
                m = i-d.get(&s).unwrap();
            }
        }
        else {d.insert(s,i);}
    }
    m as u32
}