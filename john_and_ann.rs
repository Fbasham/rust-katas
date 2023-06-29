fn john(n: i32) -> Vec<i32> {
    let mut a = vec![1];
    let mut j = vec![0];
    for i in 1..n {
        j.push(i - a[j[i as usize - 1] as usize]);
        a.push(i - j[a[i as usize - 1] as usize]);
    }
    j
}

fn ann(n: i32) -> Vec<i32> {
    let mut a = vec![1];
    let mut j = vec![0];
    for i in 1..n {
        j.push(i - a[j[i as usize - 1] as usize]);
        a.push(i - j[a[i as usize - 1] as usize]);
    }
    a
}

fn sum_john(n: i32) -> i32 {
    john(n).iter().sum()
}

fn sum_ann(n: i32) -> i32 {
    ann(n).iter().sum()
}
