use std::collections::*;

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut d: HashMap<usize, usize> = HashMap::new();
    let mut m: HashMap<i32, char> = HashMap::new();
    m.insert(0, '0');
    let mut v: VecDeque<usize> = VecDeque::new();
    for (i, c) in code.chars().enumerate() {
        match c {
            '[' => v.push_back(i),
            ']' => {
                let k = v.pop_back().unwrap();
                d.insert(k, i);
                d.insert(i, k);
            }
            _ => (),
        }
    }
    let mut inp = input
        .into_iter()
        .rev()
        .map(|c| format!("{:08b}", c).chars().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    let mut out: Vec<char> = Vec::new();
    let mut i = 0;
    let mut p = 0;
    while i < code.len() {
        let e = code.as_bytes()[i] as char;
        if e == '+' {
            if m.get(&p).unwrap() == &'0' {
                m.insert(p, '1');
            } else {
                m.insert(p, '0');
            }
        }
        if e == ';' {
            out.push(*m.get(&p).unwrap());
        }
        if e == ',' {
            match inp.pop() {
                Some(n) => _ = m.insert(p, n),
                None => _ = m.insert(p, '0'),
            }
        }
        if e == '>' {
            p = p + 1;
            if !m.contains_key(&p) {
                _ = m.insert(p, '0')
            }
        }
        if e == '<' {
            p = p - 1;
            if !m.contains_key(&p) {
                _ = m.insert(p, '0')
            }
        }
        if e == '[' {
            if m.get(&p).unwrap() == &'0' {
                i = *d.get(&i).unwrap()
            }
        }
        if e == ']' {
            if m.get(&p).unwrap() != &'0' {
                i = *d.get(&i).unwrap()
            }
        }
        i = i + 1;
    }
    for _ in 0..out.len() % 8 {
        out.push('0')
    }
    out.as_slice()
        .chunks(8)
        .map(|chunk| u8::from_str_radix(&chunk.iter().rev().collect::<String>(), 2).unwrap())
        .collect()
}
