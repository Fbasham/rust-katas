use itertools::Itertools;

fn find_routes<S>(a: &[[S; 2]]) -> String
where
    S: AsRef<str> + std::fmt::Debug + std::fmt::Display,
{
    let v = a
        .iter()
        .map(|e| e.iter().map(|x| x.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let d = v.iter().flatten().unique().collect::<Vec<_>>();
    for c in d.iter().cloned() {
        let mut r = vec![];
        let mut p = c.to_string();
        while r.len() != d.len() {
            let i = v
                .iter()
                .position(|e| e[0] == p.to_string())
                .unwrap_or(100000);
            if i == 100000 {
                break;
            }
            r.push(v[i][0].to_string());
            p = v[i][1].to_string();
        }
        r.push(p);
        if r.len() == d.len() {
            return r.join(", ");
        }
    }
    String::new()
}
