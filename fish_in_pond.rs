use itertools::Itertools;

fn fish(s: &String) -> u32 {
    let v = (0..100).map(|i| 2 * i * (i + 1)).collect::<Vec<_>>();
    let mut n = 1;
    let mut k = 0;
    for e in s.chars().filter(|&e| e != '0').sorted() {
        let t = e.to_digit(10).unwrap();
        let x = v.iter().position(|&i| i > k).unwrap() as u32;
        if t > x {
            break;
        }
        k += t;
    }
    v.iter().position(|&i| i > k).unwrap() as u32
}
