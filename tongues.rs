fn tongues(s: &str) -> String {
    s.chars().map(|e| {
        match e.is_alphabetic() {
            true => match "aiyeou".contains(e.to_ascii_lowercase()) {
                true => {
                    let x = "aiyeou".chars().nth(("aiyeou".chars().position(|x| x==e.to_ascii_lowercase()).unwrap()+3)%6).unwrap();
                    if e.is_lowercase() {x} else {x.to_ascii_uppercase()}
                },
                _ => {
                    let x = "bkxznhdcwgpvjqtsrlmf".chars().nth(("bkxznhdcwgpvjqtsrlmf".chars().position(|x| x==e.to_ascii_lowercase()).unwrap()+10)%20).unwrap();
                    if e.is_lowercase() {x} else {x.to_ascii_uppercase()}
                },
            },
            _ => e
        }
    }).collect()
}