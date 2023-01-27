fn solution(s: &str, x: &str) -> bool {
    match x.len()<=s.len(){
        true => &s[s.len()-x.len()..]==x,
        false => false,
    }
}