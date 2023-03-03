fn sort_by_bit(v: &mut Vec<u32>) -> Vec<u32> {
    v.sort_by(|a, b| {
        a.count_ones()
            .partial_cmp(&b.count_ones())
            .unwrap()
            .then_with(|| a.cmp(&b))
    });
    v.to_vec()
}
