fn find_deleted_number(a: &[u16], t: &[u16]) -> Option<u16> {
    a.iter().find(|e| !t.contains(e)).copied()
}
