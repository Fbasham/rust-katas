fn f(t: &str) -> i32 {
    t.split("|")
        .enumerate()
        .map(|(i, e)| {
            e.parse::<i32>().unwrap()
                * (if i == 0 {
                    3600
                } else if i == 1 {
                    60
                } else {
                    1
                })
        })
        .sum()
}

fn g(t: i32) -> String {
    let h = t / 3600;
    let m = (t % 3600) / 60;
    let s = (t % 3600) % 60;
    return format!("{:02}|{:02}|{:02}", h, m, s);
}

fn stati(s: &str) -> String {
    if s == "" {
        return "".to_string();
    }
    let mut v = s.split(", ").map(|e| f(e)).collect::<Vec<_>>();
    v.sort();
    let r = v[v.len() - 1] - v[0];
    let u = v.iter().sum::<i32>() / (v.len() as i32);
    let m = if v.len() % 2 == 1 {
        v[v.len() / 2]
    } else {
        (v[v.len() / 2] + v[v.len() / 2 - 1]) / 2
    };
    format!("Range: {} Average: {} Median: {}", g(r), g(u), g(m))
}
