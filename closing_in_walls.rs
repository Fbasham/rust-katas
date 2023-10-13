fn can_escape(a: &[(usize, usize)]) -> bool {
    for i in 0..a.len() {
        let t = a[i];
        let x = (t.0).saturating_sub(i + 1);
        let y = (t.1).saturating_sub(i + 1);
        if x == 0 || y == 0 {
            return false;
        }
    }
    true
}
