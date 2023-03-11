use std::collections::HashMap;

fn brain_luck(code: &str, mut input: Vec<u8>) -> Vec<u8> {
    let mut d = HashMap::new();
    let mut t = vec![];
    for i in 0..code.len(){
        if code.chars().nth(i).unwrap()=='[' {t.push(i)};
        if code.chars().nth(i).unwrap()==']' {
            let j = t.pop().unwrap();
            d.insert(i,j);
            d.insert(j,i);
        }
    }
    let mut r = vec![];
    let mut t = vec![0;30000];
    let mut i = 0;
    let mut p = 0;
    while i<code.len() {
        let c = code.chars().nth(i).unwrap();
        if c=='>' {p += 1;}
        if c=='<' {p -= 1;}
        if c=='+' {t[p] = if t[p]==255 {0} else {t[p]+1};}
        if c=='-' {t[p] = if t[p]==0 {255} else {t[p]-1};}
        if c=='.' {r.push(t[p]);}
        if c==',' {t[p] = input.remove(0);}
        if c=='[' && t[p]==0 {i = *d.get(&i).unwrap();}
        if c==']' && t[p]!=0 {i = *d.get(&i).unwrap();}
        i += 1;
    }
    r
}