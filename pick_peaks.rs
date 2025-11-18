fn pick_peaks(a: &[i32]) -> (Vec<usize>, Vec<i32>) {
    let mut pos = vec![];
    let mut peaks = vec![];
    for i in 1..a.len() {
        let mut f = false;
        for j in i+1..a.len() {
            if a[i-1]>=a[i] || a[j]>a[i] {break}
            if a[j]<a[i] {
                f = true;
                break;
            }
        }
        if f {
            pos.push(i);
            peaks.push(a[i]);
        }
    }
    (pos,peaks)
}