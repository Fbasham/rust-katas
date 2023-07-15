use itertools::Itertools;

fn magic_triangle_solutions(a: &[u8; 9]) -> Vec<[u8; 9]> {
    let t = (1..10).filter(|e| !a.contains(e)).collect::<Vec<_>>();
    let mut v = vec![];
    for p in t.iter().permutations(t.len()) {
        let mut c = p.iter();
        let mut u = [0; 9];
        for i in 0..9 {
            if a[i] == 0 {
                u[i] = **c.next().unwrap();
            } else {
                u[i] = a[i];
            }
        }
        let x = &u[0..4].iter().sum::<u8>();
        let y = &u[3..7].iter().sum::<u8>();
        let z = &u[6..9].iter().sum::<u8>() + u[0];
        if x == y && x == &z {
            v.push(u)
        }
    }
    v
}
