use std::collections::HashSet;

fn path_finder(m: &str) -> bool {
    let a = m
        .split("\n")
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut q = vec![(0, 0)];
    let mut s = HashSet::new();
    while q.len() > 0 {
        let t = q.pop().unwrap();
        if t.0 == a.len() as i32 - 1 && t.1 == a[0].len() as i32 - 1 {
            return true;
        }
        s.insert(t);
        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let i = t.0 as i32 + dy;
            let j = t.1 as i32 + dx;
            if i >= 0
                && i < a.len() as i32
                && j >= 0
                && j < a[0].len() as i32
                && a[i as usize][j as usize] != 'W'
                && !s.contains(&(i, j))
            {
                q.push((i, j));
            }
        }
    }
    false
}
