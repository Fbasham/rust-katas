fn likes(a: &[&str]) -> String {
    match a.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", a[0]),
        2 => format!("{} like this", a.join(" and ")),
        3 => format!("{}, {} like this", a[0], a[1..].join(" and ")),
        _ => format!("{}, {} and {} others like this", a[0], a[1], a.len() - 2),
    }
}
