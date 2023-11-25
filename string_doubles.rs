fn doubles(s: &str) -> String {
    let mut a = s.chars().map(|e| e.to_string()).collect::<Vec<_>>();
    let mut i = 0;
    while i<a.len()-1 {
        if a[i]==a[i+1] {
            a.remove(i);
            a.remove(i);
            i = i.saturating_sub(1);
        }
        else {
            i += 1;
        }
    }
    a.join("")
}