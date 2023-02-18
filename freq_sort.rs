use itertools::Itertools;

fn solve(v: &[i32]) -> Vec<i32> {
    let d = v.iter().counts();
    v.iter().sorted_by(|a,b| d[b].cmp(&d[a]).then_with(|| a.cmp(b))).map(|e| *e).collect()
}