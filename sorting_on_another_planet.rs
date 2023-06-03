use itertools::Itertools;

fn sort_twisted_37(a: &[i8]) -> Vec<i8> {
    a.iter()
        .cloned()
        .sorted_by_key(|e| {
            e.to_string()
                .chars()
                .map(|x| match x {
                    '3' => '7',
                    '7' => '3',
                    _ => x,
                })
                .join("")
                .parse::<i8>()
                .unwrap()
        })
        .collect()
}
