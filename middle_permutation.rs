use itertools::Itertools;

pub fn middle_permutation(s: &str) -> String {
    let t = s.chars().sorted().join("");
    let x = s.len()/2-((s.len()&1)^1);
    match s.len()%2 {
        1 => format!("{}{}{}",t.chars().nth(x).unwrap(),t.chars().nth(x-1).unwrap(),(t[..x-1].to_string()+&t[x+1..]).chars().rev().join("")),
        _ => format!("{}{}",t.chars().nth(x).unwrap(),(t[..x].to_string()+&t[x+1..]).chars().rev().join(""))
    }
}