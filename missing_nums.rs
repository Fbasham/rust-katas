fn consecutive(a: &[i16]) -> i16 {
    let m = a.iter().min().unwrap_or(&1).clone();
    let n = a.iter().max().unwrap_or(&0).clone();
    (m..=n).filter(|i| !a.contains(&i)).count() as i16
}
