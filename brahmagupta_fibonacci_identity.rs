fn solve(x: Vec<i128>) -> (i128, i128) {
    let mut v = x.to_vec();
    while v.len() > 2 {
        let [a,b,c,d] = v[..4] else {break};
        v = v
            .into_iter()
            .skip(4)
            .chain([(a * c - b * d).abs(), a * d + b * c])
            .collect();
    }
    (v[0], v[1])
}
