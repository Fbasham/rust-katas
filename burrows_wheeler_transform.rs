use itertools::Itertools;

fn encode(s: &str) -> (String, usize) {
    if s=="" {return ("".to_string(),0)}
    let a = (0..s.len()).map(|i| s[s.len()-i..].to_owned()+&s[0..s.len()-i]).sorted().collect::<Vec<_>>();
    (a.iter().map(|t| &t[t.len()-1..]).join(""),a.iter().position(|t| t==s).unwrap())
}

fn decode(s: &str, n: usize) -> String {
    if s=="" {return "".to_string()}
    let mut a = (0..s.len()).map(|i| (0..s.len()).map(|_| ' ').collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    for i in 0..s.len() {
        for j in 0..s.len() {
            a[j][s.len()-i-1] = s.chars().nth(j).unwrap();
        }
        a = a.iter().cloned().sorted().collect::<Vec<Vec<char>>>();
    }
    a[n].iter().collect()
}