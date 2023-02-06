fn rgb(r: i32, g: i32, b: i32) -> String {
    let f = |n: i32| n.max(0).min(255);
    format!("{:02X}{:02X}{:02X}", f(r), f(g), f(b)).to_string()
}
