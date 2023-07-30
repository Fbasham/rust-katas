use itertools::Itertools;

fn count_vegetables(s: &str) -> Vec<(u32, String)> {
    let v = vec![
        "cabbage", "carrot", "celery", "cucumber", "mushroom", "onion", "pepper", "potato", "tofu",
        "turnip",
    ];
    let d = s.split(" ").counts();
    d.keys()
        .filter(|e| v.contains(e))
        .map(|k| (d.get(k).unwrap().clone() as u32, k.to_string()))
        .sorted_by_key(|t| ((t.0 as i32), (t.1).to_string()))
        .rev()
        .collect::<Vec<_>>()
}
