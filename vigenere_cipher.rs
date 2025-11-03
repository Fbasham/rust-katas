struct VigenèreCipher { 
    abc: Vec<char>,
    key: Vec<char>,
}

impl VigenèreCipher {
    fn new(abc: &str, key: &str) -> Self {
        Self {
            abc: abc.chars().collect(),
            key: key.chars().collect(),
        }
    }
    
    fn encode(&self, s: &str) -> String {
        let mut r = String::new();
        for (i,e) in s.chars().enumerate() {
            r = match self.abc.contains(&e) {
                true => format!("{r}{}",self.abc.iter().nth((self.abc.iter().position(|x| x==&e).unwrap()+self.abc.iter().position(|x| x==self.key.iter().nth(i%self.key.len()).unwrap()).unwrap())%self.abc.len()).unwrap()),
                _ => format!("{r}{e}"),
            }
        }
        r
    }
    
    fn decode(&self, s: &str) -> String {
        let mut r = String::new();
        for (i,e) in s.chars().enumerate() {
            r = match self.abc.contains(&e) {
                true => format!("{r}{}",self.abc.iter().nth((self.abc.len()+self.abc.iter().position(|x| x==&e).unwrap()-self.abc.iter().position(|x| x==self.key.iter().nth(i%self.key.len()).unwrap()).unwrap())%self.abc.len()).unwrap()),
                _ => format!("{r}{e}"),
            }
        }
        r
    }
}