use itertools::Itertools;

fn solve(a: &[&str]) -> String {
    let mut v = a
        .iter()
        .map(|e| e[0..2].parse::<i32>().unwrap() * 60 + e[3..5].parse::<i32>().unwrap())
        .sorted()
        .collect::<Vec<_>>();
    v.push(v[0] + 24 * 60);
    let t = v
        .iter()
        .zip(v.iter().skip(1))
        .map(|(i, j)| j - i - 1)
        .max()
        .unwrap();
    format!("{:02}:{:02}", t / 60, t % 60)
}
