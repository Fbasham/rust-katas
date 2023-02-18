use itertools::Itertools;

fn sort_array(a: &[i32]) -> Vec<i32> {
    let mut o = a.iter().filter(|&e| e % 2 == 1).sorted();
    a.iter()
        .map(|e| if e % 2 == 0 { *e } else { *o.next().unwrap() })
        .collect()
}
