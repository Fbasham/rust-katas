struct Projectile {
    h: f64,
    v: f64,
    a: f64,
}

impl Projectile {
    fn new(h: u64, v: u64, a: u64) -> Self {
        Projectile {
            h: h as f64,
            v: v as f64,
            a: (a as f64).to_radians(),
        }
    }

    fn height_eq(&self) -> String {
        let mut x = ((1000.0 * self.v * self.a.sin()).round() / 1000.0).to_string();
        if !x.contains(".") {
            x += ".0"
        }
        while x.ends_with("0") {
            x = x[0..x.len() - 1].to_string()
        }
        if x.ends_with(".") {
            x += "0"
        }
        if self.h == 0.0 {
            return format!("h(t) = -16.0t^2 + {}t", x);
        }
        if self.h % 1.0 == 0.0 {
            return format!("h(t) = -16.0t^2 + {}t + {:.1}", x, self.h);
        }
        format!("h(t) = -16.0t^2 + {}t + {:.3}", x, self.h)
    }

    fn horiz_eq(&self) -> String {
        let mut x = ((1000.0 * self.v * self.a.cos()).round() / 1000.0).to_string();
        if !x.contains(".") {
            x += ".0"
        }
        while x.ends_with("0") {
            x = x[0..x.len() - 1].to_string()
        }
        if x.ends_with(".") {
            x += "0"
        }
        format!("x(t) = {}t", x)
    }

    fn height(&self, t: f64) -> f64 {
        (1000.0 * (self.v * self.a.sin() * t - 16.0 * t * t + self.h)).round() / 1000.0
    }

    fn horiz(&self, t: f64) -> f64 {
        (1000.0 * (self.v * self.a.cos() * t)).round() / 1000.0
    }

    fn landing(&self) -> [f64; 3] {
        let a = -16.0;
        let b = self.v * self.a.sin();
        let c = self.h;
        let t = (-b - (b * b - 4.0 * a * c).sqrt()) / 2.0 / a;
        let x = self.horiz(t);
        [x, 0.0, (1000.0 * t).round() / 1000.0]
    }
}
