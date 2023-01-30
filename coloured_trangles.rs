fn triangle(s: &str) -> String {
    let mut t = s.to_string();
    while t.len() > 1 {
        t = (0..t.len() - 1)
            .map(|i| match &t[i..i + 2] {
                "RR" => "R",
                "GG" => "G",
                "BB" => "B",
                "BG" => "R",
                "GB" => "R",
                "RG" => "B",
                "GR" => "B",
                "BR" => "G",
                "RB" => "G",
                _ => "",
            })
            .collect();
    }
    t.to_string()
}
