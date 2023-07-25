use num::complex::Complex;

fn g(z: Complex<f64>, x: i32) -> Complex<f64> {
    let n = Complex::new(1.0, 0.0) - z;
    let mut d = Complex::new(0.0, 0.0);
    let mut m = Complex::new(1.0, 0.0);
    for i in 0..x {
        m *= z;
        d += m;
    }
    n * d
}

fn f(z: Complex<f64>, eps: f64) -> i32 {
    let lim = g(z, 10000);
    for i in 0..10000 {
        if (g(z, i) - lim).norm() < eps {
            return i;
        }
    }
    -1
}
