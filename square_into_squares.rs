fn decompose(n: i64) -> Option<Vec<i64>> {
    let mut r: Vec<Vec<i64>> = vec![];
    for m in (n-3..n).rev() {
        let mut q = vec![(n*n,m,vec![])];
        while q.len()>0 {
            let (mut x, k, mut t) = q.pop().unwrap();
            if t.contains(&k) {continue;}
            t.push(k);
            x -= k*k;
            if x==0 {r.push(t.clone().into_iter().collect());}
            let v = (x as f64).sqrt() as i64;
            for i in ((v-2).max(1)..=v).rev() {
                if i<k {q.insert(0,(x,i,t.clone()));}
            }
        }
    }
    r.sort();
    if r.len()>0 {Some(r[r.len()-1].clone().into_iter().rev().collect())} else {None}
}