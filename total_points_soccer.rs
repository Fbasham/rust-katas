fn points(a: &[String]) -> u32 {
    a.iter().map(|s| {
        let mut t = s.split(":");
        let x = t.next().unwrap().parse::<u32>().unwrap();
        let y = t.next().unwrap().parse::<u32>().unwrap();
        if x==y {1} else if x>y {3} else {0} 
    }).sum()
}