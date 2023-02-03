use std::collections::HashMap;

fn to_leet_speak(s: &str) -> String {
    let d = HashMap::from([
        ('A', '@'),
        ('B', '8'),
        ('C', '('),
        ('D', 'D'),
        ('E', '3'),
        ('F', 'F'),
        ('G', '6'),
        ('H', '#'),
        ('I', '!'),
        ('J', 'J'),
        ('K', 'K'),
        ('L', '1'),
        ('M', 'M'),
        ('N', 'N'),
        ('O', '0'),
        ('P', 'P'),
        ('Q', 'Q'),
        ('R', 'R'),
        ('S', '$'),
        ('T', '7'),
        ('U', 'U'),
        ('V', 'V'),
        ('W', 'W'),
        ('X', 'X'),
        ('Y', 'Y'),
        ('Z', '2'),
    ]);
    s.chars()
        .map(|k| match d.get(&k) {
            Some(v) => *v,
            None => k,
        })
        .collect()
}
