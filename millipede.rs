use itertools::Itertools;

fn millipede(a: &[&str]) -> bool {
    for p in a.iter().permutations(a.len()) {
        if p.iter()
            .zip(p.iter().skip(1))
            .all(|(x, y)| x.chars().last().unwrap() == y.chars().nth(0).unwrap())
        {
            return true;
        }
    }
    false
}
