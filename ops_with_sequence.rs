fn calc(a: Vec<i32>) -> i32 {
    a.iter().cloned().enumerate().map(|(i,e)| {
        let mut x = e;
        if e>0 {x = x*x;}
        if (i+1)%3==0 {x = 3*x}
        if (i+1)%5==0 {x = -x}
        x
    }).sum()
}