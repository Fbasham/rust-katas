fn find_part_max_prod(n: u32) -> (Vec<Vec<u32>>, u32) {
    let mut t = vec![0; n as usize];
    t[0] = n;
    let mut k: isize = 0;
    let mut m = 0;
    let mut r = vec![];
    loop {
        let mut x = 0;
        while k >= 0 && t[k as usize] == 1 {
            x += t[k as usize];
            k -= 1;
        }
        if k < 0 {
            break;
        }
        t[k as usize] -= 1;
        x += 1;
        while x > t[k as usize] {
            t[k as usize + 1] = t[k as usize];
            x = x - t[k as usize];
            k += 1;
        }
        t[k as usize + 1] = x;
        k += 1;
        let u: Vec<u32> = t.iter().cloned().filter(|&e| e > 1).collect();
        let p: u32 = u.iter().product();
        if p == m {
            r.push(u.iter().cloned().collect());
        }
        if p > m {
            m = p;
            r = vec![u];
        }
    }
    (r, m)
}
