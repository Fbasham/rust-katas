fn quadratic_formula(y1: i32, y2: i32, y3: i32) -> (String, i32, i32) {
    let a = (y3 - 2 * y2 + y1) / 2;
    let b = y2 - y1 - 3 * a;
    let c = y1 - a - b;
    let v = vec![(a, "x^2"), (b, "x"), (c, "c")]
        .iter()
        .filter(|t| t.0 != 0)
        .map(|(i, j)| match (i, j) {
            (_, &"c") => i.to_string(),
            (1, _) => j.to_string(),
            (-1, _) => format!("-{}", j),
            _ => format!("{}{}", i, j),
        })
        .collect::<Vec<_>>();
    let r = v.join("+").replace("+-", "-");
    (r, a * 16 + b * 4 + c, a * 25 + b * 5 + c)
}
