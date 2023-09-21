mod irc {
    pub fn encode(n: u32, st: &str) -> String {
        let mut r = st.to_string();
        for _ in 0..n {
            let p = (0..r.len())
                .filter(|&i| r.chars().nth(i).unwrap() == ' ')
                .collect::<Vec<_>>();
            let mut s = r.replace(" ", "");
            s = s[s.len() - n as usize..].to_owned() + &s[0..s.len() - n as usize];
            let mut t = s.split("").collect::<Vec<_>>();
            for i in &p {
                t.insert(*i + 1, " ");
            }
            let v = t
                .join("")
                .split(" ")
                .map(|e| {
                    e[e.len() - (n as usize % e.len().max(1))..].to_owned()
                        + &e[0..e.len() - (n as usize % e.len().max(1))]
                })
                .collect::<Vec<_>>();
            r = v.join(" ");
        }
        format!("{n} {r}")
    }

    pub fn decode(s: &str) -> String {
        let (n, r) = s.split_once(" ").unwrap();
        let n = n.parse::<usize>().unwrap();
        let mut r = r.to_string();
        for _ in 0..n {
            let p = (0..r.len())
                .filter(|&i| r.chars().nth(i).unwrap() == ' ')
                .collect::<Vec<_>>();
            let t = r
                .split(" ")
                .map(|e| {
                    e[n as usize % e.len().max(1)..].to_owned()
                        + &e[0..(n as usize % e.len().max(1))]
                })
                .collect::<Vec<_>>();
            let mut s = t.join("");
            s = s[n..].to_owned() + &s[0..n];
            let mut v = s.split("").collect::<Vec<_>>();
            for i in &p {
                v.insert(*i + 1, " ");
            }
            r = v.join("");
        }
        r
    }
}
