fn stanton_measure(a: &[u32]) -> u32 {
    let x = a.iter().filter(|&e| e == &(1 as u32)).count();
    a.iter().filter(|&e| e == &(x as u32)).count() as u32
}
