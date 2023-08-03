use itertools::Itertools;

fn f(n: u32) -> bool {
    if n%2==0 {return n==2}
    for i in (3..=(n as f64).sqrt() as u32).step_by(2) {
        if n%i==0 {return false}
    }
    n>1
}

fn g(n: u32) -> u32 {
    (1..=n).filter(|e| n%e==0).count() as u32
}

fn proc_arr_int(a: &[u32]) -> (u32, u32, (u32, Vec<u32>)) {
    let p = a.iter().cloned().filter(|e| f(*e)).collect::<Vec<_>>();
    let mut m = 0;
    for e in a {
        let d = g(*e);
        m = m.max(d);  
    }
    (a.len() as u32,p.len() as u32,(m,a.iter().cloned().filter(|e| g(*e)==m).sorted().collect()))
}