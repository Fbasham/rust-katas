fn merge_strings(a: &str, b: &str) -> String {
    let i = (0..a.len()).position(|i| b.starts_with(&a[i..]));
    if let Some(i) = i {
        a[..i].to_owned() + b
    } else {
        a.to_owned() + b
    }
}
