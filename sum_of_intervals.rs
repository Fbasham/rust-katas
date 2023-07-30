use itertools::Itertools;

fn sum_intervals(a: &[(i32, i32)]) -> i32 {
    let mut v: Vec<(i64,i64)> = vec![];
    for (i,j) in a.iter().cloned().sorted_by_key(|k| (k.0,k.1)).map(|(i,j)| (i as i64,j as i64)) {
        let mut f = true;
        for x in 0..v.len() {
            let t = v[x];
            if (i<=t.0 && j>=t.0) || (j>=t.1 && i<=t.1) || (i>=t.0 && j<=t.1) {
                v[x] = (i.min(t.0),j.max(t.1));
                f = false;
                break;
            }
        }
        if f {v.push((i,j))}
    }
    v.iter().fold(0,|a,c| a+c.1-c.0) as i32
}