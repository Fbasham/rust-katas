fn remove_parentheses(s: &str) -> String {
    let mut q = vec![];
    let mut v = vec![];
    for (i, e) in s.chars().enumerate() {
        if e == '(' {
            q.push(i)
        }
        if e == ')' && q.len() > 0 {
            v.push((q.pop().unwrap(), i));
        }
    }
    let mut w = vec![];
    for t in v {
        if w.len() == 0 {
            w.push(t);
            continue;
        }
        let mut f = true;
        for i in 0..w.len() {
            let x = w[i];
            if (t.0 < x.0 && t.1 >= x.0) || (t.1 > x.1 && t.0 <= x.0) {
                w[i] = ((t.0).min(w[i].0), (t.1).max(w[i].1));
                f = false;
                break;
            }
        }
        if f {
            w.push(t);
        }
    }
    w.sort_by_key(|t| t.0);
    let mut d = 0;
    let mut r = s.to_string();
    for t in w {
        let i = t.0 - d;
        let j = t.1 - d;
        r = (r[0..i].to_owned() + &r[j + 1..]).to_string();
        d += j - i + 1;
    }
    r
}
