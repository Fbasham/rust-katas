fn max_hexagon_beam(n: u8, a: &[i32]) -> i32 {
    let mut h = vec![];
    let mut c = a.iter().cycle();
    for i in n..2*n {h.push((0..i).map(|_| c.next().unwrap()).collect::<Vec<_>>());}
    for i in (n..2*n-1).rev() {h.push((0..i).map(|_| c.next().unwrap()).collect::<Vec<_>>());}
    let mut m = h.iter().map(|t| t.iter().cloned().sum::<i32>()).max().unwrap() as i32;
    for j in 0..2*n as usize-1 {
        let mut t = 0;
        for i in 0..n as usize {
            if j<h[i].len() {t += h[i][j];} 
            if i>0 &&i<=j {t += h[n as usize-1+i][j-i];}
        }
        m = m.max(t);
    }    
    for j in 0..2*n as usize-1 {
        let mut t = 0;
        for i in 0..n as usize {
            if j<h[n as usize-1+i].len() {t += h[n as usize-1+i][j];} 
            if i>0 &&i<=j {t += h[n as usize-1-i][j-i];}
        }
        m = m.max(t);
    }
    m
}