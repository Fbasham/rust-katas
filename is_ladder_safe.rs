use itertools::Itertools;

fn is_ladder_safe(v: &[&str]) -> bool {
    let mut d = v
        .iter()
        .enumerate()
        .map(|(i, e)| (i, e, format!("#{}#", " ".repeat(e.len().saturating_sub(2)))))
        .filter(|t| t.1 != &&t.2)
        .map(|t| t.0)
        .collect::<Vec<_>>();
    if d.len() == 0
        || v[0] == "#".repeat(v[0].len())
        || v[v.len() - 1] == "#".repeat(v[v.len() - 1].len())
        || !d.iter().all(|i| v[*i] == "#".repeat(v[*i].len()))
    {
        return false;
    }
    d = d.iter().zip(&d[1..]).map(|t| t.1 - t.0).collect();
    v.iter().map(|e| e.len()).unique().count() == 1
        && v.iter().all(|e| e.len() > 4)
        && d.iter().unique().count() == 1
        && d.iter().all(|e| e < &4)
}
