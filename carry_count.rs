fn solve(s: &str) -> String {
    s.split("\n").map(|e| {
        let mut t = e.split(" ");
        let a = t.next().unwrap();
        let b = t.next().unwrap();
        let mut c = 0;
        let mut k = 0;
        for i in (0..a.len()).rev(){
            let x = &a[i..i+1].parse::<i32>().unwrap()+&b[i..i+1].parse::<i32>().unwrap()+c;
            k += if x>9 {1} else {0};
            c = x/10;
        }
        format!("{} carry operation{}",if k==0 {"No".to_string()} else {k.to_string()},if k==0 {"".to_string()} else {"s".to_string()})
    }).collect::<Vec<_>>().join("\n")
}