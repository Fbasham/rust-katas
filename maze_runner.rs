fn maze_runner(a: &[Vec<i32>], d: &[&str]) -> &'static str {
    let mut y = a.iter().position(|t| t.contains(&2)).unwrap() as i32;
    let mut x = a[y as usize].iter().position(|x| x==&2).unwrap() as i32;
    for s in d {
        y += match s {&"N"=>-1,&"S"=>1,_=>0};
        x += match s {&"W"=>-1,&"E"=>1,_=>0};
        if y<0 || y as usize==a.len() || x<0 || x as usize>=a[0].len() || a[y as usize][x as usize]==1 {return "Dead"}
        if a[y as usize][x as usize]==3 {return "Finish"}
    }
    "Lost"
}