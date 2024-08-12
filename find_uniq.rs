fn find_uniq(a: &[f64]) -> f64 {
    let mut v = a.to_vec();
    v.dedup();
    *v.iter().find(|e| a.iter().filter(|x| x==e).count()==1).unwrap()
}