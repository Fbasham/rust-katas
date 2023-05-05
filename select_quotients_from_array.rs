use itertools::Itertools;

fn select_quotients(a: &[u32], m: u32, d: &str) -> Vec<(u32, (u32, u32))> {
    a.iter()
        .sorted()
        .rev()
        .combinations(2)
        .unique()
        .filter(|v| {
            v[0] % v[1] == 0
                && v[0] / v[1] >= m
                && (d == ""
                    || (if d.to_lowercase() == "odd" {
                        v[0] / v[1] % 2 == 1
                    } else if d.to_lowercase() == "even" {
                        v[0] / v[1] % 2 == 0
                    } else {
                        false
                    }))
        })
        .map(|v| (v[0] / v[1], (*v[0], *v[1])))
        .sorted_by_key(|t| (t.0, t.1))
        .collect()
}
