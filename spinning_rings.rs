fn spinning_rings (i: u8, o: u8) -> u8 {
    let mut x = i as i8;
    let mut y = 1 as i8;
    let mut c = 1;
    while x != y {
        x = x-1;
        y = y+1;
        c += 1;
        if x<0 {x = i as i8;}
        if y>o as i8 {y = 0;}
    }
    c
}