fn plane_seat(s: &str) -> String {
    let n = s[..s.len() - 1].parse::<usize>().unwrap();
    let k = &s[s.len() - 1..];
    if n < 1 || n > 60 || "IJ".contains(k) {
        return "No Seat!!".to_string();
    }
    let p = "Front Middle Back".split(" ").nth((n - 1) / 20).unwrap();
    let d = "Left Middle Right"
        .split(" ")
        .nth(
            "ABC DEF GHK"
                .split(" ")
                .position(|e| e.contains(k))
                .unwrap(),
        )
        .unwrap();
    format!("{p}-{d}")
}
