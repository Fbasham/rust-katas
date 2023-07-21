use itertools::Itertools;

struct S {
    s: String,
    xs: Vec<i32>,
}

fn sqr_modulus(a: S) -> (bool, i32, i32) {
    if a.s == "polar" {
        let r = (0..a.xs.len())
            .step_by(2)
            .map(|i| a.xs[i].pow(2))
            .sum::<i32>();
        let t = r
            .to_string()
            .chars()
            .sorted()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();
        return (true, r, t);
    }
    if a.s == "cart" {
        let r = a.xs.iter().map(|e| e * e).sum::<i32>();
        let t = r
            .to_string()
            .chars()
            .sorted()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();
        return (true, r, t);
    }
    (false, -1, 1)
}
