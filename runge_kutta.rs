fn rk4(x0: f64, y0: f64, h: f64, f: fn(f64, f64) -> f64, x1: f64) -> Vec<f64> {
    let n = ((x1-x0)/h) as i32;
    let mut y = y0;
    let mut v = vec![y];
    for i in 0..n {
        let x = x0+i as f64*h;
        let k1 = h*f(x,y);
        let k2 = h*f(x+h/2.0,y+k1/2.0);
        let k3 = h*f(x+h/2.0,y+k2/2.0);
        let k4 = h*f(x+h,y+k3);
        y = y+(k1+2.0*k2+2.0*k3+k4)/6.0;
        v.push(y);
    }
    v
}