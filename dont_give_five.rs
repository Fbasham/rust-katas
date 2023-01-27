fn dont_give_me_five(m: isize, n: isize) -> isize {
    (m..=n)
        .filter(|e| e.to_string().matches("5").count() == 0)
        .count()
        .try_into()
        .unwrap()
}
