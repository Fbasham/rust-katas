fn distance_from_line(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> f64 {
    if a==b {return ((a.0-c.0).powf(2.0)+(a.1-c.1).powf(2.0)).sqrt()}
    ((b.0-a.0)*(a.1-c.1)-(a.0-c.0)*(b.1-a.1)).abs()/((b.0-a.0).powf(2.0)+(b.1-a.1).powf(2.0)).sqrt()
}