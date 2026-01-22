use std::collections::HashMap;

#[derive(Debug)]
pub struct SnakesLadders {
    v: Vec<usize>,
    k: usize,
    w: bool,
    d: HashMap<usize,usize>,
}

impl SnakesLadders {
    pub fn new() -> Self {
        Self {
            v: vec![0,0],
            k: 0,
            w: false,
            d: HashMap::from([
                (2,38),(7,14),(8,31),(15,26),(16,6),(21,42),(28,84),(36,44),(46,25),(49,11),
                (51,67),(62,19),(64,60),(71,91),(74,53),(78,98),(87,94),(89,68),(92,88),(95,75),(99,80)
            ])
        }
    }

    pub fn play(&mut self, d1: u8, d2: u8) -> String {
        if self.w {return format!("Game over!")}
        let x = self.v[self.k]+d1 as usize+d2 as usize;
        self.v[self.k] = *self.d.get(&x).unwrap_or(&x);
        if self.v[self.k]>100 {
            self.v[self.k] = 100-self.v[self.k]%100;
            self.v[self.k] = *self.d.get(&self.v[self.k]).unwrap_or(&self.v[self.k]);
        }
        let p = self.k;
        self.k = if d1==d2 {self.k} else {self.k^1};
        if self.v[p]==100 {
            self.w = true;
            return format!("Player {} Wins!",p+1);
        }
        format!("Player {} is on square {}",p+1,self.v[p])
    }
}
