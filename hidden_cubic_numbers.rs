use regex::Regex;

fn is_sum_of_cubes(s: &str) -> String {
    let v = Regex::new(r"\d{1,3}")
        .unwrap()
        .find_iter(s)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .filter(|&e| {
            e.to_string()
                .chars()
                .map(|e| ((e as i32) - 48).pow(3))
                .sum::<i32>()
                == e
        })
        .collect::<Vec<_>>();
    match v.len() > 0 {
        true => format!(
            "{} {} Lucky",
            v.iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" "),
            v.iter().sum::<i32>()
        ),
        false => "Unlucky".to_string(),
    }
}
