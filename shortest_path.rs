use std::collections::HashSet;

fn path_finder(s: &str) -> Option<u32> {
    let a = s.split("\n").map(|t| t.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut v = HashSet::new();
    let mut q = vec![(0 as i32,0 as i32,0)];
    while q.len()>0 {
        let (y,x,c) = q.pop().unwrap();
        if y as usize==a.len()-1 && x as usize==a.len()-1 {return Some(c)}
        if v.contains(&(y,x)) {continue}
        v.insert((y,x));
        for (i,j) in [(y-1,x),(y+1,x),(y,x-1),(y,x+1)] {
            if i>=0 && (i as usize)<a.len() && j>=0 && (j as usize)<a.len() && a[i as usize][j as usize]=='.' {
                q.insert(0,(i,j,c+1));
            }
        }
    }
    None
}