use itertools::Itertools;

fn score(a: [u8; 5]) -> u32 {
    let d = a.into_iter().map(|e| e as u32).counts();
    d.into_iter().map(|(k,v)| match (k,v) {
        (1,_) => (1000*(v/3)+100*(v%3)) as u32,
        (5,_) => 100*(v as u32/3)*k+50*(v as u32%3),
        _ => (if v>2 {k*100} else {0}) as u32
    }).sum()
}