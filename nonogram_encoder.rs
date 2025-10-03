use itertools::Itertools;

type Clues = Vec<Vec<u8>>;

fn encode(a: &[&[u8]]) -> (Clues, Clues) {
    let t: Clues = a
        .iter()
        .map(|t| {
            t.iter()
                .map(|e| e.to_string())
                .join("")
                .split("0")
                .map(|e| e.len() as u8)
                .filter(|&e| e > 0)
                .collect()
        })
        .collect();
    let r: Clues = (0..a[0].len())
        .map(|i| {
            a.iter()
                .map(|t| t[i])
                .map(|e| e.to_string())
                .join("")
                .split("0")
                .map(|e| e.len() as u8)
                .filter(|&e| e > 0)
                .collect()
        })
        .collect();
    (r, t)
}
