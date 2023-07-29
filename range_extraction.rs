mod solution {
    use itertools::Itertools;
    pub fn range_extraction(a: &[i32]) -> String {
        let mut v = vec![];
        let mut r = vec![];
        for e in a {
            if v.len() < 1 || v.len() > 0 && e - v[v.len() - 1] == 1 {
                v.push(e)
            } else {
                if v.len() < 2 {
                    r.push(v[..1].to_vec());
                    v = v[1..].to_vec();
                } else if v.len() == 2 {
                    r.push(vec![v[0]]);
                    r.push(vec![v[1]]);
                    v = vec![];
                } else {
                    r.push(v.to_vec());
                    v = vec![];
                }
                v.push(e);
            }
        }
        if v.len() == 2 {
            r.push(vec![v[0]]);
            r.push(vec![v[1]]);
        } else {
            r.push(v.to_vec());
        }
        r.iter()
            .map(|t| {
                if t.len() == 1 {
                    t[0].to_string()
                } else {
                    format!("{}-{}", t[0], t[t.len() - 1])
                }
            })
            .join(",")
    }
}
