fn som(x: i64, y: i64) -> i64 {
    x + y
}
fn maxi(x: i64, y: i64) -> i64 {
    x.max(y)
}
fn mini(x: i64, y: i64) -> i64 {
    x.min(y)
}
fn gcdi(m: i64, n: i64) -> i64 {
    if n.abs() > 0 {
        gcdi(n, m % n)
    } else {
        m.abs()
    }
}
fn lcmu(a: i64, b: i64) -> i64 {
    (a * b / gcdi(a.abs(), b.abs())).abs()
}

fn oper_array(f: fn(i64, i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {
    let mut v = vec![];
    for e in a {
        if v.is_empty() {
            v.push(f(init, *e))
        } else {
            v.push(f(v[v.len() - 1], *e))
        }
    }
    v
}
