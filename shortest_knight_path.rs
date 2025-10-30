fn knight(p1: &str, p2: &str) -> u8 {
    let x1 = p1.chars().nth(0).unwrap() as i8-96;
    let y1 = p1.chars().nth(1).unwrap() as i8-48;
    let x2 = p2.chars().nth(0).unwrap() as i8-96;
    let y2 = p2.chars().nth(1).unwrap() as i8-48;
    let mut q = vec![(x1,y1,0)];
    let mut s = vec![];
    while q.len()>0 {
        let (x,y,z) = q.pop().unwrap();
        if x<1 || y<1 || x>8 || y>8 || s.contains(&(x,y)) {continue}
        if (x,y)==(x2,y2) {return z}
        s.push((x,y));
        for (x,y) in [(x-2,y-1),(x-1,y-2),(x-1,y+2),(x-2,y+1),(x+2,y-1),(x+1,y-2),(x+2,y+1),(x+1,y+2)] {
            q.insert(0,(x,y,z+1));
        }
    }
    0
}