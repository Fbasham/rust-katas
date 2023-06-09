fn next_higher(n: i32) -> i32 {
    let k = format!("{n:b}").chars().filter(|&e| e=='1').count();
    for i in 1.. {
        if format!("{:b}",n+i).chars().filter(|&e| e=='1').count()==k {
            return n+i
        }
    }
    0
}