use itertools::Itertools;

fn calc_poly(a: &[i32], x: i32) -> String {
    let mut y: i32 = a.iter().enumerate().map(|(i,e)| e*x.pow((a.len()-i) as u32-1)).sum();
    format!("For {} with x = {x} the value is {y}",a.iter().enumerate().map(|(i,e)| (a.len()-i-1,e)).map(|(i,e)| match (e,i) {
        (0,_) => format!(""),
        (_,0) => format!("{e}"),
        (1,1) => format!("x"),
        (-1,1) => format!("-x"),
        (_,1) => format!("{e}*x"),
        (1,_) => format!("x^{i}"),
        (-1,_) => format!("-x^{i}"),
        _ => format!("{e}*x^{i}")
    }).filter(|e| e!="").join(" + ").replace("+ -","- "))
    
}