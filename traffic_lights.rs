use itertools::Itertools;

fn traffic_lights(s: &str, n: usize) -> String {
    let mut v = s.chars().collect::<Vec<_>>();
    let mut w = vec![0;v.len()];
    let mut p = v.iter().position(|&e| e=='C').unwrap();
    v[p] = '.';
    let mut r = vec![];
    for _ in 0..n.min(v.len()) {
        r.push(v.iter().enumerate().map(|(i,e)| if i==p {'C'} else {*e}).join(""));
        for j in 0..v.len() {
            if "RGO".contains(v[j]) {
                w[j] += 1;
                if "RG".contains(v[j]) && w[j]>4 {
                    v[j] = if v[j]=='G' {'O'} else {'G'};
                    w[j] = 0;
                }
                if v[j]=='O' && w[j]>0 {
                    v[j] = 'R';
                    w[j] = 0;
                }
            }
        }
        if p+1<v.len() && v[p+1]=='R' {continue;}
        p += 1;
    }
    r.join("\n")
}