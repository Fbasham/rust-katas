use itertools::Itertools;

fn best_match(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .enumerate()
        .map(|(i, e)| (i, e, b[i]))
        .sorted_by_key(|k| (if k.1 > &k.2 { k.1 - k.2 } else { k.2 - k.1 }, -(k.2 as i8)))
        .next()
        .unwrap()
        .0
}
