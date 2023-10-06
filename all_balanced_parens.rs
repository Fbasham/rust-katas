fn balanced_parens(n: u16) -> Vec<String> {
    let mut r = vec![];
    let mut q = vec![(n,0,String::new())];
    while q.len()>0 {
        let (x,y,s) = q.pop().unwrap();
        if x==0 && y==0 {r.push(s.to_string())}
        if x>0 {q.push((x-1,y+1,s.to_string()+"("))}
        if y>0 {q.push((x,y-1,s.to_string()+")"))}
    }
    r
}