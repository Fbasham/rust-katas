fn to_time(s: u32) -> String {
    format!("{} hour(s) and {} minute(s)", s / 3600, (s % 3600) / 60)
}
