use itertools::Itertools;

fn count_different_matrices(m: &[[i16; 4]]) -> usize {
    m.iter()
        .map(|[a, b, c, d]| {
            vec![
                format!("{a} {b} {c} {d}"),
                format!("{c} {a} {d} {b}"),
                format!("{d} {c} {b} {a}"),
                format!("{b} {d} {a} {c}"),
            ]
            .iter()
            .sorted()
            .join(" ")
        })
        .unique()
        .count()
}
