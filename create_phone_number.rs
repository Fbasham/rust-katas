fn create_phone_number(a: &[u8]) -> String {
    let s = a.iter().map(|e| e.to_string()).collect::<String>();
    format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
}
