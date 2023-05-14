fn f(mut n: i32) -> i32 {
    let mut c = 0;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            c += 1;
        }
        i += 1;
    }
    if n > 1 {
        c += 1;
    }
    c
}

fn count_kprimes(k: i32, a: i32, b: i32) -> Vec<i32> {
    (a..=b).filter(|&e| f(e) == k).collect()
}

fn puzzle(s: i32) -> i32 {
    let mut c = 0;
    for i in 1..s {
        for j in 1..s - i {
            let k = s - i - j;
            if f(i) == 1 && f(j) == 3 && f(k) == 7 {
                c += 1;
            }
        }
    }
    c
}
