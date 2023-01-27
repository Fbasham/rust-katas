fn mx_dif_lg(a: Vec<&str>, b: Vec<&str>) -> i32 {
    if a.len() < 1 || b.len() < 1 {
        return -1;
    }
    let ma = a.iter().map(|e| e.len()).min().unwrap();
    let xa = a.iter().map(|e| e.len()).max().unwrap();
    let mb = b.iter().map(|e| e.len()).min().unwrap();
    let xb = b.iter().map(|e| e.len()).max().unwrap();
    ma.abs_diff(xb).max(mb.abs_diff(xa)) as i32
}
