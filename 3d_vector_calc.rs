#[derive(Copy, Clone)]
struct Vector {
    i: f64,
    j: f64,
    k: f64,
}

impl Vector {
    fn new(i: f64, j: f64, k: f64) -> Self {
        Vector { i: i, j: j, k: k }
    }
    fn get_magnitude(&self) -> f64 {
        (self.i * self.i + self.j * self.j + self.k * self.k).sqrt()
    }
    fn get_i() -> Vector {
        Vector::new(1.0, 0.0, 0.0)
    }
    fn get_j() -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
    fn get_k() -> Vector {
        Vector::new(0.0, 0.0, 1.0)
    }
    fn add(&self, other: Vector) -> Vector {
        Vector::new(self.i + other.i, self.j + other.j, self.k + other.k)
    }
    fn multiply_by_scalar(&self, k: f64) -> Vector {
        Vector::new(self.i * k, self.j * k, self.k * k)
    }
    fn dot(&self, other: Vector) -> f64 {
        self.i * other.i + self.j * other.j + self.k * other.k
    }
    fn cross(&self, other: Vector) -> Vector {
        Vector::new(
            self.j * other.k - self.k * other.j,
            self.k * other.i - self.i * other.k,
            self.i * other.j - self.j * other.i,
        )
    }
    fn is_parallel_to(&self, other: Vector) -> bool {
        let u = self.cross(other);
        !(self.i == 0.0 && self.j == 0.0 && self.k == 0.0)
            && !(other.i == 0.0 && other.j == 0.0 && other.k == 0.0)
            && u.i.abs() < 1e-6
            && u.j.abs() < 1e-6
            && u.k.abs() < 1e-6
    }
    fn is_perpendicular_to(&self, other: Vector) -> bool {
        !(self.i == 0.0 && self.j == 0.0 && self.k == 0.0)
            && !(other.i == 0.0 && other.j == 0.0 && other.k == 0.0)
            && self.dot(other).abs() < 1e-6
    }
    fn normalize(&self) -> Vector {
        let m = self.get_magnitude();
        Vector::new(self.i / m, self.j / m, self.k / m)
    }
    fn is_normalized(&self) -> bool {
        (1.0 - self.get_magnitude()).abs() < 1e-6
    }
}
