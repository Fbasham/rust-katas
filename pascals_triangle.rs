fn f(n: f64) -> f64{
    if n<2.0 {1.0} else {n*f(n-1.0)}
}

fn pascals_triangle(n: usize) -> Vec<usize> {
    let c = |n,k| (f(n as f64)/f(k as f64)/f((n-k) as f64)).round();
    (0..n).map(|i| (0..=i).map(move |j| c(i,j) as usize)).flat_map(|e| e).collect()
}