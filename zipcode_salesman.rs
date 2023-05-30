fn travel(s: &str, z: &str) -> String {
    let mut a = vec![];
    let mut d = vec![];
    let t = s
        .split(",")
        .filter(|&e| z != "" && e.ends_with(z))
        .for_each(|e| {
            let q = e.trim().split(" ").collect::<Vec<_>>();
            a.push(q[1..q.len() - 2].join(" "));
            d.push(q[0]);
        });
    format!("{}:{}/{}", z, a.join(","), d.join(","))
}
