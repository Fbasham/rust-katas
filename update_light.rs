fn update_light(s: &str) -> String {
    (match s {
        "green" => "yellow",
        "yellow" => "red",
        _ => "green",
    })
    .to_string()
}
