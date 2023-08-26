sfn quadratic(a: f64, b: f64, c: f64) -> f64 {
    let x1 = (-b+(b*b-4.0*a*c).sqrt())/2.0/a;
    let x2 = (-b-(b*b-4.0*a*c).sqrt())/2.0/a;
    c/a/x1.min(x2)
}