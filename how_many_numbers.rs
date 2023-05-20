use itertools::Itertools;

fn sel_number(n: u32, d: u8) -> u8 {
    (12..=n)
        .filter(|&i| {
            i.to_string()
                .chars()
                .tuple_windows()
                .all(|(i, j)| j > i && (j as u8) - (i as u8) <= d)
        })
        .count() as u8
}
