use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;

fn rank<T: Clone + Hash + Ord>(a: &[T]) -> Vec<f64> {
    let mut d = HashMap::new();
    let v = a.iter().cloned().sorted().collect::<Vec<_>>();
    for i in 0..v.len() {
        if !d.contains_key(&v[i]) {d.insert(v[i].clone(),vec![]);}
        d.get_mut(&v[i]).unwrap().push(i);
    }
    a.iter().cloned().map(|k| {
        let t = d.get(&k).unwrap();
        t.iter().sum::<usize>() as f64/t.len() as f64
    }).collect()
}ss