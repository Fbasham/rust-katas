use itertools::Itertools;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn from_nb_2str(n: i64, v: Vec<i64>) -> String {
    let p: i64 = v.iter().product();
    let g = v
        .iter()
        .cloned()
        .combinations(2)
        .any(|t| gcd(t[0], t[1]) != 1);
    if p < n || g {
        return "Not applicable".to_string();
    }
    v.iter().map(|e| format!("-{}-", n % e)).join("")
}
