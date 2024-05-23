fn encode(s: &str) -> String {
    s.chars()
        .map(|e| match e {
            'G' => 'A',
            'g' => 'a',
            'A' => 'G',
            'a' => 'g',
            'D' => 'E',
            'd' => 'e',
            'E' => 'D',
            'e' => 'd',
            'R' => 'Y',
            'r' => 'y',
            'Y' => 'R',
            'y' => 'r',
            'P' => 'O',
            'p' => 'o',
            'O' => 'P',
            'o' => 'p',
            'L' => 'U',
            'l' => 'u',
            'U' => 'L',
            'u' => 'l',
            'K' => 'I',
            'k' => 'i',
            'I' => 'K',
            'i' => 'k',
            _ => e,
        })
        .collect()
}

fn decode(s: &str) -> String {
    encode(s)
}
