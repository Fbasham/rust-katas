fn encode(s: &str) -> String {
    let t = s
        .chars()
        .map(|e| format!("{:08b}", e as u8))
        .collect::<String>();
    t.chars()
        .map(|e| e.to_string().repeat(3))
        .collect::<String>()
}

fn decode(s: &str) -> String {
    let a = s.chars().collect::<Vec<_>>();
    let t = &a
        .chunks(3)
        .map(|e| {
            let z = e.into_iter().filter(|&x| x == &'0').count();
            let o = e.into_iter().filter(|&x| x == &'1').count();
            if z > o {
                '0'
            } else {
                '1'
            }
        })
        .collect::<Vec<_>>();
    t.chunks(8)
        .map(|e| {
            u8::from_str_radix(&e.iter().collect::<String>(), 2)
                .ok()
                .unwrap() as char
        })
        .collect::<String>()
}
