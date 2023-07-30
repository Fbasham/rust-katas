use itertools::Itertools;

fn mix(s1: &str, s2: &str) -> String {
    (s1.to_owned() + s2)
        .chars()
        .unique()
        .filter(|&e| e >= 'a' && e <= 'z')
        .map(|e| {
            let x = s1.chars().filter(|&x| x == e).count();
            let y = s2.chars().filter(|&x| x == e).count();
            let z = if x == y {
                "3"
            } else if x > y {
                "1"
            } else {
                "2"
            };
            (e, x.max(y), z)
        })
        .filter(|t| t.1 > 1)
        .sorted_by_key(|t| (-(t.1 as i32), t.2, t.0))
        .map(|t| {
            format!(
                "{}:{}",
                if t.2 == "3" { "=" } else { t.2 },
                (0..t.1).map(|_| t.0).join("")
            )
        })
        .join("/")
}
