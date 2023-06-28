fn guess<F>(f: F, n: usize) -> Vec<i32>
where
    F: Fn(usize, usize) -> i32,
{
    let mut v = vec![];
    let mut p = 0;
    let mut q = 0;
    for i in 0..n - 2 {
        let x = f(i, i + 1);
        let y = f(i + 1, i + 2);
        let z = f(i, i + 2);
        let a = (x + z - y) / 2;
        p = x - a;
        q = z - a;
        v.push(a);
    }
    v.push(p);
    v.push(q);
    v
}
