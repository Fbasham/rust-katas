fn unpack_sausages(truck: Vec<Vec<&str>>) -> String {
    let mut v = vec![];
    let mut k = 0;
    for p in truck {
        for s in p {
            let mut t = s.split("").collect::<Vec<_>>();
            if t.len() < 4 {
                continue;
            }
            t = t[2..t.len() - 2].to_vec();
            if s.starts_with("[") && s.ends_with("]") && t.join("") == t[0].repeat(4) {
                k += 1;
                if k % 5 == 0 {
                    continue;
                }
                for c in t {
                    v.push(c.to_string());
                }
            }
        }
    }
    v.join(" ")
}
