use std::collections::HashMap;
use lazy_static::lazy_static;
use itertools::Itertools;

lazy_static! {
    static ref abc: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    static ref V: Vec<String> = 
        (0..1).map(|_| abc.chars()).multi_cartesian_product().map(|e| e.iter().collect()).chain(
        (0..2).map(|_| abc.chars()).multi_cartesian_product().map(|e| e.iter().collect())).chain(
        (0..3).map(|_| abc.chars()).multi_cartesian_product().map(|e| e.iter().collect())).chain(
        (0..4).map(|_| abc.chars()).multi_cartesian_product().map(|e| e.iter().collect())).collect();
}

struct UrlShortener {
    n: usize,
    d: HashMap<String,String>
}

impl UrlShortener {
    fn new() -> Self {
        Self {
            n: 0,
            d: HashMap::new()
        }
    }
    
    fn shorten(&mut self, s: &str) -> String {
        if self.d.contains_key(s) {
            return self.d.get(s).unwrap().to_string();
        }
        let v = format!("short.ly/{}",V[self.n]);
        self.d.insert(s.to_string(),v.to_string());
        self.d.insert(v.to_string(),s.to_string());
        self.n += 1;
        self.d.get(&s.to_string()).unwrap().to_string()
    }
    
    fn redirect(&self, s: &str) -> String {
        self.d.get(&s.to_string()).unwrap().to_string()
    }
}