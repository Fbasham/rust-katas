fn split_unique_substrings(s: &str) -> Vec<usize> {
    let d = s.chars().enumerate().map(|(i,e)| (e,i)).collect::<HashMap<char,usize>>();
    let mut x = 0;
    let mut y = 0;
    let mut v = vec![];
    for (i,k) in s.chars().enumerate() {
        y = y.max(*d.get(&k).unwrap());
        if i==y {
            v.push(y-x+1);
            x = i+1;
        }
    }
    v
}