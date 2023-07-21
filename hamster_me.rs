use itertools::Itertools;

fn hamster_me(code: &str, msg: &str) -> String {
    let mut v = vec![];
    for c in code.chars().sorted() {
        let mut t = c.to_string();
        for i in 0..26 {
            let k = ((c as u8 - 97 + i + 1) % 26 + 97) as char;
            if code.contains(k) {
                v.push(t);
                break;
            }
            t += &k.to_string();
        }
    }
    msg.chars()
        .map(|e| {
            let i = v.iter().position(|x| x.contains(e)).unwrap();
            format!(
                "{}{}",
                &v[i][..1],
                v[i].chars().position(|x| x == e).unwrap() + 1
            )
        })
        .collect()
}
