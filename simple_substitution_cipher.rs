struct Cipher {
    a: &'static str,
    b: &'static str,
}

impl Cipher {
    fn new(a: &'static str, b: &'static str) -> Cipher {
        Cipher {
            a: a,
            b: b,
        }
    }

    fn encode(&self, s: &str) -> String {
        s.chars().map(|e| self.b.chars().nth(self.a.chars().position(|x| e==x).unwrap_or(100000)).unwrap_or(e)).collect()
    }

    fn decode(&self, s: &str) -> String {
        s.chars().map(|e| self.a.chars().nth(self.b.chars().position(|x| e==x).unwrap_or(100000)).unwrap_or(e)).collect()
    }
}