fn is_valid_ip(ip: &str) -> bool {
    !ip.is_empty()
        && ip.split(".").count() == 4
        && ip
            .split(".")
            .all(|e| e == "0" || !e.starts_with("0") && e.parse::<u32>().unwrap_or(256) < 256)
}
