use std::ops::Sub;

fn delta<I, T>(a: I, n: usize) -> impl Iterator<Item = T> 
where
    I: IntoIterator<Item = T>,
    T: Sub<Output = T> + Copy + std::fmt::Debug,
{
    let mut it = a.into_iter();
    let mut r = vec![];
    while r.len()<1100 {
        if let Some(v) = it.next(){
            r.push(v);
        }
        else {break}
    }
    for x in 0..n {
        r = (1..r.len()).map(|i| r[i]-r[i-1]).collect();
    }
    r.into_iter()
}
