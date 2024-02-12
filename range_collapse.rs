use itertools::Itertools;

fn descriptions(a: &[u32]) -> u32 {
    let d = a[..a.len() - 1]
        .iter()
        .cloned()
        .zip_eq(a[1..].iter().cloned())
        .map(|(i, j)| j - i)
        .collect::<Vec<_>>();
    let t = d
        .iter()
        .cloned()
        .group_by(|k| k.clone())
        .into_iter()
        .map(|(_, g)| g.collect::<Vec<u32>>())
        .collect::<Vec<_>>();
    t.iter()
        .cloned()
        .filter(|t| t.contains(&1))
        .map(|t| 2_u32.pow(t.len() as u32))
        .product()
}
