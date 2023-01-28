use itertools::Itertools;

fn max_tri_sum(a: &[i32]) -> i32 {
    let v = a.iter().unique().map(|&e| e).collect::<Vec<i32>>();
    let mut m = i32::MIN;
    for i in 0..v.len() - 2 {
        for j in i + 1..v.len() - 1 {
            for k in j + 1..v.len() {
                let n = v[i] + v[j] + v[k];
                if n > m {
                    m = n;
                }
            }
        }
    }
    m
}
