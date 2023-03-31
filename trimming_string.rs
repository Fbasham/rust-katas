fn trim(s: &str, k: usize) -> String {
    match s.len()<=k {
        true => s.to_string(),
        _ => s[0..if k>3 {k-3} else {k}].to_string()+"..."
    }
}