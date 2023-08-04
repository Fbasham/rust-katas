use itertools::Itertools;

fn rearranger(k: u32, a: &[u16]) -> String {
    let mut v = vec![];
    let mut m = u128::MAX;
    for p in a.iter().permutations(a.len()) {
        let t = p.iter().map(|e| e.to_string()).collect::<String>().parse::<u128>();
        match t {
            Ok(t) => if t%(k as u128)==0 {
                v.push((t,p.iter().map(|e| e.to_string()).join(", ")));
                m = m.min(t);
            },
            _ => ()
        }
    }
    if v.len()==0 {return "There is no possible rearrangement".to_string()}
    let u = v.iter().filter(|t| t.0==m).unique().collect::<Vec<_>>();
    format!(
        "Rearrangement{}: {} generate{}: {} divisible by {}",
        if u.len()==1 {""} else {"s"},
        u.iter().map(|t| t.1.to_string()).sorted().join(" and "),
        if u.len()==1 {"s"} else {""},
        m,k
    )
}