fn format_duration(t: u64) -> String {
    let y = t / 365 / 24 / 60 / 60;
    let d = (t % (365 * 24 * 60 * 60)) / 24 / 60 / 60;
    let h = (t % (365 * 24 * 60 * 60)) % (24 * 60 * 60) / 60 / 60;
    let m = (t % (365 * 24 * 60 * 60)) % (24 * 60 * 60) % (60 * 60) / 60;
    let s = (t % (365 * 24 * 60 * 60)) % (24 * 60 * 60) % (60 * 60) % 60;
    let v = [
        (y, "year"),
        (d, "day"),
        (h, "hour"),
        (m, "minute"),
        (s, "second"),
    ]
    .iter()
    .filter(|t| t.0 > 0)
    .map(|(x, y)| format!("{} {}{}", x, y, if x == &1 { "" } else { "s" }))
    .collect::<Vec<_>>();
    match v.len() {
        0 => "now".to_string(),
        1 => v.join(""),
        2 => v.join(" and "),
        _ => format!(
            "{}, {}",
            &v[..v.len() - 2].join(", "),
            &v[v.len() - 2..].join(" and ")
        ),
    }
}
