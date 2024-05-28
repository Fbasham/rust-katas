struct CaesarCipher {
    shift: u32,
}

impl CaesarCipher {
    fn new(shift: u32) -> Self {
        Self { shift }
    }

    fn encode(&self, s: &str) -> String {
        s.to_uppercase()
            .chars()
            .map(|e| {
                if e >= 'A' && e <= 'Z' {
                    (((e as i32 - 65 + self.shift as i32) % 26) + 65) as u8 as char
                } else {
                    e
                }
            })
            .collect()
    }

    fn decode(&self, s: &str) -> String {
        s.to_uppercase()
            .chars()
            .map(|e| {
                if e >= 'A' && e <= 'Z' {
                    ((((e as i32 - 65 - self.shift as i32) % 26 + 26) % 26) + 65) as u8 as char
                } else {
                    e
                }
            })
            .collect()
    }
}
