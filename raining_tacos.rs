use itertools::Itertools;

fn rain_tacos(s: &str) -> String {
    let mut v = s
        .split("\n")
        .map(|e| e.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut t = "TACO";
    for j in 0..v[0].len() {
        let c = t.chars().nth(j % 4).unwrap();
        for i in 0..v.len() {
            if v[i][j] != ' ' {
                if i > 0 {
                    v[i - 1][j] = c
                }
                break;
            }
            if i == v.len() - 1 {
                v[i][j] = c
            }
        }
    }
    v.iter().map(|e| e.iter().join("")).join("\n")
}
