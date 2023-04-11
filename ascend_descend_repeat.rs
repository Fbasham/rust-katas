fn ascend_descend(k: usize, min: i32, max: i32) -> String {
    if k<1 || max<min {return String::new();}
    let s = (min..=max).chain((min+1..max).rev()).map(|e| e.to_string()).collect::<String>();
    s.repeat(k)[..k].to_string()
}