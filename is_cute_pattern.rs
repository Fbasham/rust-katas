use itertools::Itertools;

fn cute_pattern(s: &str) -> bool {
    let v = s.split("\n").collect::<Vec<_>>();
    for i in 0..3 {
        for j in 0..3 {
            let t = v[i..i + 2].iter().map(|t| &t[j..j + 2]).join("");
            if t.chars().unique().count() == 1 {
                return false;
            }
        }
    }
    true
}
