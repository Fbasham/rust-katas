fn frog_jumping(a: &[u32]) -> String {
    let mut q = vec![(a.iter().cloned().rev().collect::<Vec<_>>(),String::new())];
    while !q.is_empty() {
        let (t,s) = q.pop().unwrap();
        if t.len()<2 {
            return s.chars().rev().collect();
        }
        if t.len()>2 && t[0]-t[2]==2 {
            q.push((t[2..].to_vec(),s+"2"));
        }
        else if t[0]-t[1]<=2 {
            q.push((t[1..].to_vec(),s+&(t[0]-t[1]).to_string()))
        };
    }
    String::new()
}
