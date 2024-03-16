use itertools::Itertools;

fn latest_clock(a: u8, b: u8, c: u8, d: u8) -> String {
    [a, b, c, d]
        .iter()
        .permutations(4)
        .map(|t| {
            let h = format!("{}{}", t[0], t[1]).parse::<i8>().unwrap();
            let m = format!("{}{}", t[2], t[3]).parse::<i8>().unwrap();
            if h < 24 && m < 60 {
                format!("{h:02}:{m:02}")
            } else {
                String::new()
            }
        })
        .filter(|e| e != "")
        .sorted()
        .last()
        .unwrap()
}
