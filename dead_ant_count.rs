fn dead_ant_count(s: &str) -> u32 {
    let t = s.replace("ant", " ");
    "ant".chars().map(|e| t.matches(e).count()).max().unwrap() as u32
}
