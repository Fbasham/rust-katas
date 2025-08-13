use itertools::Itertools;

pub fn to_base64(data: &[u8]) -> String {
    let abc = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .chars()
        .collect::<Vec<_>>();
    let s = data.iter().map(|k| format!("{:08b}", k)).join("");
    (0..s.len())
        .step_by(6)
        .map(|i| {
            abc[usize::from_str_radix(&format!("{:0<6}", &s[i..(i + 6).min(s.len())]), 2).unwrap()]
        })
        .join("")
}

pub fn from_base64(encoded: &str) -> Vec<u8> {
    let abc = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .chars()
        .collect::<Vec<_>>();
    let s = encoded
        .chars()
        .map(|k| format!("{:06b}", abc.iter().position(|&e| e == k).unwrap()))
        .join("");
    (0..s.len() / 8)
        .map(|i| u8::from_str_radix(&s[i * 8..i * 8 + 8], 2).unwrap())
        .collect()
}
