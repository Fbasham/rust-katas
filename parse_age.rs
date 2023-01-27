fn get_age(s: &str) -> u32 {
    s.split(" ").next().unwrap().parse().unwrap()
}
