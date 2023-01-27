fn printer_error(s: &str) -> String {
    let c = s.chars().filter(|&e| match e {
        'n'..='z' => true,
        _ => false,
    });
    format!("{}/{}", c.count(), s.len())
}
