use itertools::Itertools;
use std::collections::HashMap;

fn decomp(n: i32) -> String {
    let mut d = HashMap::new();
    for mut x in 2..=n {
        let mut i = 2;
        while i * i <= n {
            while x % i == 0 {
                d.insert(i, d.get(&i).unwrap_or(&0) + 1);
                x /= i;
            }
            i += 1;
        }
        if x > 1 {
            d.insert(x, d.get(&x).unwrap_or(&0) + 1);
        }
    }
    d.iter()
        .sorted_by_key(|t| t.0)
        .map(|(k, v)| match v {
            1 => k.to_string(),
            _ => format!("{k}^{v}"),
        })
        .collect::<Vec<_>>()
        .join(" * ")
}
