fn sum_dif_rev(n: usize) -> usize {
    let mut v = vec![];
    let mut i = 45;
    while v.len() < n {
        let j = i
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        if i != j && i % 10 != 0 && (i + j) % (i - j).abs() == 0 {
            v.push(i)
        }
        i += 1;
    }
    v[v.len() - 1] as usize
}
