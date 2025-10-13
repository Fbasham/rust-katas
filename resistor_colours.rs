use std::collections::HashMap;

fn decode_resistor_colors(s: &str) -> String {
    let d = HashMap::from([
        ("black", 0),
        ("brown", 1),
        ("red", 2),
        ("orange", 3),
        ("yellow", 4),
        ("green", 5),
        ("blue", 6),
        ("violet", 7),
        ("gray", 8),
        ("white", 9),
    ]);
    let a = s.split(" ").collect::<Vec<_>>();
    let n = format!("{}{}", d.get(a[0]).unwrap(), d.get(a[1]).unwrap())
        .parse::<i32>()
        .unwrap()
        * 10_i32.pow(*d.get(a[2]).unwrap() as u32);
    return format!(
        "{}{} ohms, {}%",
        if n < 1000 {
            n as f64
        } else if n < 1000000 {
            n as f64 / 1000.0
        } else {
            n as f64 / 1000000.0
        },
        if n < 1000 {
            ""
        } else if n < 1000000 {
            "k"
        } else {
            "M"
        },
        if a.len() < 4 {
            20
        } else if a[3] == "gold" {
            5
        } else {
            10
        }
    );
}
