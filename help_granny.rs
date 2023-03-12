use std::collections::*;

fn tour(a: &[&str], m: HashMap<&str, &str>, d: HashMap<&str, f64>) -> i32 {
    let u = a.iter().filter(|&k| m.contains_key(k)).collect::<Vec<_>>();
    let t = u
        .iter()
        .zip(&u[1..])
        .map(|(i, j)| {
            ((d.get(m.get(*j).unwrap()).unwrap_or(&0.0)).powf(2.0)
                - (d.get(m.get(*i).unwrap()).unwrap_or(&0.0)).powf(2.0))
            .sqrt()
        })
        .sum::<f64>()
        .floor() as i32;
    t + *d
        .get(m.get(u.iter().nth(0).unwrap().to_owned()).unwrap())
        .unwrap() as i32
        + *d.get(
            m.get(u.iter().nth(u.len() - 1).unwrap().to_owned())
                .unwrap(),
        )
        .unwrap() as i32
}
