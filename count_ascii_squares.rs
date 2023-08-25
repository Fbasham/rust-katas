use itertools::Itertools;

fn count_squares(u: &[&str]) -> i32 {
    let mut v = vec![];
    for i in 0..u.len() {
        for j in 0..u[i].len() {
            if u[i].chars().nth(j).unwrap() == '+' {
                v.push((i as i32, j as i32));
            }
        }
    }
    let mut r = 0;
    for c in v.iter().combinations(4) {
        for p in c.iter().permutations(4) {
            let a = p[0];
            let b = p[1];
            let c = p[2];
            let d = p[3];
            if a.0 != b.0 || c.0 != d.0 {
                break;
            }
            if a.1 != c.1 || b.1 != d.1 {
                break;
            }
            let ab = ((a.0) - (b.0)).abs() + ((a.1) - (b.1)).abs();
            let ac = ((a.0) - (c.0)).abs() + ((a.1) - (c.1)).abs();
            let bd = ((b.0) - (d.0)).abs() + ((b.1) - (d.1)).abs();
            let cd = ((c.0) - (d.0)).abs() + ((c.1) - (d.1)).abs();
            if ab == cd && ac == bd && ab == bd && ac == ab {
                r += 1;
            }
        }
    }
    r
}
