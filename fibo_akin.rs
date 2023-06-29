fn u(n: i32) -> Vec<i32> {
    let mut v = vec![1, 1];
    for i in 2..n as usize {
        v.push(v[i - v[i - 1] as usize] + v[i - v[i - 2] as usize] as i32);
    }
    v
}

fn length_sup_uk(n: i32, k: i32) -> i32 {
    u(n).iter().filter(|&e| e >= &k).count() as i32
}

fn comp(n: i32) -> i32 {
    let v = u(n);
    (1..v.len()).filter(|&i| v[i] < v[i - 1]).count() as i32
}
