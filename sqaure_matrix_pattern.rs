use itertools::Itertools;

fn make_matrix(m: u32, n:u32) -> String {
    let a = m.to_string().chars().collect::<Vec<_>>();
    let s = format!("{}{}{}{}{}",a[0],a[1],a[3],a[2],a[4]);
    let mut v = (0..n).map(|_| (0..n).map(|_| s.to_string().chars().nth(0).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    for e in 1..5 {
        for i in 0..n/2 {
            for j in 0..n-2*(i+1) {
                v[i as usize][(i+1+j) as usize] = s.to_string().chars().nth(e).unwrap();
            }
            
        }
        v = (0..n).map(|i| v.iter().map(|t| t[t.len()-1-i as usize]).collect()).collect();
    }
    v.iter().map(|t| t.iter().rev().join(" ")).join("\n")
}