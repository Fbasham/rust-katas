use num::Complex;

fn easy_polynomials(n: i32) -> Complex<f64> {
    let t = 2.0*std::f64::consts::PI/(n as f64+1.0);
    Complex::new(t.cos(),t.sin())
}