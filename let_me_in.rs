use itertools::Itertools;

fn get_in_line(a: &mut Vec<i32>) -> i32 {
    let mut c = 0;
    let mut v = vec![
        a.iter()
            .cloned()
            .filter(|&e| e == 1 || e == 2)
            .sorted()
            .collect::<Vec<_>>(),
        a.iter()
            .cloned()
            .filter(|&e| e != 1 && e != 2)
            .collect::<Vec<_>>(),
    ]
    .into_iter()
    .concat();
    let n = v.len();
    for i in 0..n {
        c += 1;
        if v[i] == 0 {
            break;
        }
        if v[i] == 1 {
            for j in 1..=(n - i) / 2 {
                if v[j + i] == 1 || v[j + i] == 3 || v[n - j] == 1 || v[n - j] == 3 {
                    continue;
                }
                (v[j + i], v[n - j]) = (v[n - j], v[j + i]);
            }
        }
    }
    c
}
