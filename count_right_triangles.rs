use itertools::Itertools;

fn f(p: &Vec<&(i32, i32)>) -> bool {
    let a = ((p[0].0 - p[1].0) as f64).powf(2.0) + ((p[0].1 - p[1].1) as f64).powf(2.0);
    let b = ((p[0].0 - p[2].0) as f64).powf(2.0) + ((p[0].1 - p[2].1) as f64).powf(2.0);
    let c = ((p[1].0 - p[2].0) as f64).powf(2.0) + ((p[1].1 - p[2].1) as f64).powf(2.0);
    let v = vec![a, b, c]
        .iter()
        .map(|e| *e as i32)
        .sorted()
        .collect::<Vec<_>>();
    v[0] + v[1] == v[2]
}

fn count_right_triangles(a: &[(i32, i32)]) -> u32 {
    a.iter().unique().combinations(3).filter(|p| f(p)).count() as u32
}
