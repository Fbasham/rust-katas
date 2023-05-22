fn compare_versions(s1: &str, s2: &str) -> bool {
    let a = s1.split(".").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let b = s2.split(".").map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();
    a.iter().zip(&b).all(|(i,j)| i>=j) && a.len()>=b.len()
}