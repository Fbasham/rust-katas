use itertools::Itertools;
use std::collections::HashMap;

fn valid(a: Vec<Vec<&str>>) -> bool {
    if a.iter().map(|t| t.join("").len()).unique().count()!=1 {return false}
    if !a.iter().all(|t| t.iter().map(|x| x.len()).unique().count()==1) {return false}
    let mut d = HashMap::new();
    for v in a {
        for x in v {
            for k in x.chars() {
                if !d.contains_key(&k) {d.insert(k,String::new());}
                d.insert(k,format!("{}{}",d.get(&k).unwrap(),x.replace(k,"")));
            }
        }
    } 
    if d.values().map(|t| t.len()).unique().count()!=1 {return false}
    if d.values().any(|t| t.len()!=t.chars().unique().count()) {return false}
    true
}