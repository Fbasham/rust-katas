fn get_middle(s:&str) -> &str {
    &s[s.len()/2-(if s.len()&1==1 {0} else {1})..s.len()/2+1]
}