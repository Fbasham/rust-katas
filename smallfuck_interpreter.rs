use itertools::Itertools;
use std::collections::HashMap;

fn interpreter(code: &str, tape: &str) -> String {
    let mut d = HashMap::new();
    let mut t = vec![];
    code.chars().enumerate().for_each(|(i, e)| match e {
        '[' => t.push(i),
        ']' => {
            let j = t.pop().unwrap();
            d.insert(i, j);
            d.insert(j, i);
        }
        _ => (),
    });
    let mut i = 0;
    let mut p: i32 = 0;
    let mut a: Vec<u8> = tape.chars().map(|e| if e == '0' { 0 } else { 1 }).collect();
    while i < code.len() && p >= 0 && p < a.len() as i32 {
        let e = code.chars().nth(i).unwrap();
        match e {
            '>' => p += 1,
            '<' => p -= 1,
            '*' => a[p as usize] ^= 1,
            '[' if a[p as usize] == 0 => i = *d.get(&i).unwrap() - 1,
            ']' if a[p as usize] != 0 => i = *d.get(&i).unwrap(),
            _ => (),
        };
        i += 1;
    }
    a.iter().map(|e| e.to_string()).join("")
}
