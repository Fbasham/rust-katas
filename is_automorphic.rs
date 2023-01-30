fn automorphic(n: u64) -> String {
    match (n * n).to_string().ends_with(n.to_string().as_str()) {
        true => "Automorphic".to_string(),
        false => "Not!!".to_string(),
    }
}
