fn frog_contest(n: u32) -> String {
    let f = |n| n*(n+1)/2;
    let x = f(n);
    let y = f(x/2);
    format!("Chris ate {x} flies, Tom ate {y} flies and Cat ate {} flies",f(x+y))
}