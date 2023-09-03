fn create_box(m: u32, n: u32) -> Vec<Vec<u32>> {
    let mut v = vec![];
    for i in 1..=n/2+(if n%2==1 {1} else {0}) {
        let mut t = (1..=m/2+(if m%2==1 {1} else {0})).map(|x| x.min(i)).collect::<Vec<_>>();
        v.push(t.iter().cloned().chain(t.iter().cloned().rev().skip(if m%2==1 {1} else {0})).collect()); 
    }
    v.iter().cloned().chain(v.iter().cloned().rev().skip(if n%2==1 {1} else {0})).collect()
}