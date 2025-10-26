fn f(n: i32, a: &[u32]) -> i32 {
    if n<0 || a.len()==0 {0} else if n==0 {1} else {f(n-a[a.len()-1] as i32,a)+f(n,&a[..a.len()-1])}
}

fn count_change(money: u32, coins: &[u32]) -> u64 {
    if money==0 && coins.len()==0 {1} else {f(money as i32,coins) as u64}
}