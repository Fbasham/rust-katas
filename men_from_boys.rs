use itertools::Itertools;

fn men_from_boys(a: &[i16]) -> Vec<i16> {
    a.iter()
        .filter(|&e| e % 2 == 0)
        .unique()
        .sorted()
        .chain(
            a.iter()
                .filter(|&e| (e % 2).abs() == 1)
                .unique()
                .sorted()
                .rev(),
        )
        .map(|e| *e)
        .collect()
}
