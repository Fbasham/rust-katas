fn leaderboard_sort(a: &[String], c: &[String]) -> Vec<String> {
    let mut v = a.to_vec();
    for e in c {
        let t = e.split(" ").collect::<Vec<_>>();
        let n = t[0];
        let k = t[1].parse::<i32>().unwrap();
        let i = v.iter().position(|x| x==n).unwrap();
        v.remove(i);
        v.insert((i as i32-k) as usize,n.to_string());
    }
    v
}