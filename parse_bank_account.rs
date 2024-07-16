fn parse_bank_account(s: &str) -> u64 {
    let a = [
        " _  | | |_|",
        "      |   |",
        " _   _| |_ ",
        " _   _|  _|",
        "    |_|   |",
        " _  |_   _|",
        " _  |_  |_|",
        " _    |   |",
        " _  |_| |_|",
        " _  |_|  _|",
    ];
    let t = s.trim_end_matches("\n").split("\n").collect::<Vec<_>>();
    (0..t[0].len())
        .step_by(3)
        .map(|i| {
            let v = t.iter().map(|e| &e[i..i + 3]).collect::<Vec<_>>().join(" ");
            a.iter()
                .position(|w| w.to_string() == v)
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}
