use itertools::Itertools;

fn pendulum(a: &[i32]) -> Vec<i32> {
    let mut v = vec![];
    for (i, e) in a.iter().cloned().sorted().enumerate() {
        if i % 2 == 1 {
            v.push(e);
        } else {
            v.insert(0, e);
        }
    }
    v
}
