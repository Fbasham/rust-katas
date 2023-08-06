struct Quark {
    color: &'static str,
    flavor: &'static str,
}

impl Quark {
    pub fn new(color: &'static str, flavor: &'static str) -> Quark {
        Quark { color, flavor }
    }
    pub fn color(&self) -> &str {
        self.color
    }
    pub fn flavor(&self) -> &str {
        self.flavor
    }
    pub fn baryon_number(&self) -> f64 {
        1.0 / 3.0
    }
    pub fn interact(&mut self, q2: &mut Quark) {
        (self.color, q2.color) = (q2.color, self.color)
    }
}
