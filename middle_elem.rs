fn gimme(a: [i32; 3]) -> usize {
    let mut x = a.clone();
    x.sort();
    a.iter().position(|&e| e == x[1]).unwrap()
}
