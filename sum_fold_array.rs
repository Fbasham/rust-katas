use itertools::Itertools;
use itertools::EitherOrBoth::*;

fn fold_array(a: &[i32], k: usize) -> Vec<i32> {
    let mut v = a.to_vec();
    for _ in 0..k {
        let n = v.len()/2;
        v = v[..n].iter().zip_longest(v[n..].iter().rev()).map(|x| match x {
            Both(&i,&j) => i+j,
            Left(&i) => i,
            Right(&i) => i,
        }).collect();
    }
    v
}