mod dnb {
    pub fn dots_and_boxes(a: Vec<(usize,usize)>) -> (usize,usize) {
        let n = (a.iter().cloned().map(|e| (e.0).max(e.1)).max().unwrap() as f64+1.0).sqrt() as usize;
        let mut t = vec![];
        for i in 0..n-1 {
            for j in 0..n-1 {
                t.push(vec![(i*n+j,i*n+j+1),((i+1)*n+j,(i+1)*n+j+1),(i*n+j,(i+1)*n+j),(i*n+j+1,(i+1)*n+j+1)]);
            }
        }
        let mut r = vec![0,0];
        let mut k = 0;
        for u in a {
            let mut f = false;
            for i in 0..t.len() {
                if t[i].iter().any(|&e| e==u || e.1==u.0 && e.0==u.1) {
                    t[i] = t[i].iter().cloned().filter(|&e| !(e == u || e.1==u.0 && e.0==u.1)).collect();
                    if t[i].len()==0 {
                        f = true;
                        r[k] += 1;
                    }
                }
            }
            if f {continue;}
            k ^= 1;
        }
        (r[0],r[1])
    }
}