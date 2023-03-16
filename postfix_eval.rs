fn postfix_evaluator(s: &str) -> i64 {
    let mut a = vec![];
    for e in s.split(" ") {
        if !"+-*/".contains(e) {
            a.push(e.parse::<i64>().unwrap());
        } else {
            let n = a.pop().unwrap();
            let m = a.pop().unwrap();
            let x = match e {
                "+" => m + n,
                "-" => m - n,
                "*" => m * n,
                _ => m / n,
            };
            a.push(x);
        }
    }
    a.pop().unwrap()
}
