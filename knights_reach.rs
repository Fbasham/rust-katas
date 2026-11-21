use std::collections::HashSet;

fn many_horse(x: u8, y: u8, n: usize) -> usize {
    let mut s = HashSet::from([(x as isize,y as isize)]);
    for _ in 0..n {
        let mut t = HashSet::new();
        for (x,y) in &s {
            for (i,j) in [(x-2,y-1),(x-1,y-2),(x-2,y+1),(x-1,y+2),(x+2,y-1),(x+1,y-2),(x+2,y+1),(x+1,y+2)]{
                if i>0 && i<9 && j>0 && j<9 {
                    t.insert((i,j));
                }
            }
        }
        s = t;
    }
    s.len()
}