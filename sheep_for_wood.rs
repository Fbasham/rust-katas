use std::collections::HashMap;

fn canBuild<'a>(mut a: Vec<&str>, mut p2: Vec<&'a str>, p3: Vec<&'a str>, t: &str) -> bool {
    let mut v = HashMap::from([
        ("City", vec!["Stone", "Stone", "Wheat", "Wheat", "Wheat"]),
        ("Settlement", vec!["Brick", "Wood", "Wheat", "Sheep"]),
        ("Road", vec!["Brick", "Wood"]),
    ])
    .get(t)
    .cloned()
    .unwrap();
    let mut k = a.clone();
    for e in k {
        if v.contains(&e) {
            let i = v.iter().position(|&x| x == e).unwrap();
            v.remove(i.clone());
            a.remove(i.clone());
            if v.is_empty() {
                return true;
            }
        }
    }
    p2.extend(p3);
    k = v.clone();
    for e in k {
        if p2.contains(&e) {
            p2.remove(p2.iter().position(|&x| x == e).unwrap());
            v.remove(v.iter().position(|&x| x == e).unwrap());
            if v.is_empty() {
                return true;
            }
        }
    }
    false
}
