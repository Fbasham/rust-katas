fn sixteen_circles(r: usize) -> f32 {
    let a = ((2*r*r) as f32).sqrt();
    let b = (3.0*(r as f32).powf(2.0)).sqrt();
    ((a+b-2.0*r as f32)*100.0).round()/100.0
}