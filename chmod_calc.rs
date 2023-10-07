use itertools::Itertools;
use std::collections::HashMap;

fn chmod_calculator(d: &HashMap<&str, &str>) -> String {
    ["user", "group", "other"]
        .iter()
        .map(|e| {
            d.get(e)
                .unwrap_or(&"---")
                .chars()
                .map(|e| match e {
                    'r' => 4,
                    'w' => 2,
                    'x' => 1,
                    _ => 0,
                })
                .sum::<u8>()
                .to_string()
        })
        .join("")
}
