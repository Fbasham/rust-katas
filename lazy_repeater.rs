fn make_looper(s: &str) -> impl FnMut() -> char + '_ {
    let mut i = -1;
    move || {
        i = (i+1)%s.len() as i32;
        s.chars().nth(i as usize).unwrap()
    }
}