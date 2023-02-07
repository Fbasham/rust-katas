fn rake_garden(s: &str) -> String {
    s.split_whitespace()
        .map(|e| match e {
            "gravel" | "rock" => e,
            _ => "gravel",
        })
        .collect::<Vec<&str>>()
        .join(" ")
}
