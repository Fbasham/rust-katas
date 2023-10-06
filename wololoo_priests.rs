use std::collections::HashMap;

fn alphabet_war(s: &str) -> &'static str {
    let mut v = format!(" {s} ").chars().collect::<Vec<_>>();
    let d = HashMap::from([
        ('w', 'm'),
        ('p', 'q'),
        ('b', 'd'),
        ('s', 'z'),
        ('m', 'w'),
        ('q', 'p'),
        ('d', 'b'),
        ('z', 's'),
    ]);
    let m = HashMap::from([
        ('w', -4),
        ('p', -3),
        ('b', -2),
        ('s', -1),
        ('m', 4),
        ('q', 3),
        ('d', 2),
        ('z', 1),
    ]);
    for i in 1..v.len() - 1 {
        if v[i - 1] == 't' && v[i + 1] != 'j' || v[i - 1] != 'j' && v[i + 1] == 't' {
            v[i] = if "wpbst".contains(v[i]) {
                v[i]
            } else {
                *d.get(&v[i]).unwrap_or(&v[i])
            };
        }
        if v[i - 1] == 'j' && v[i + 1] != 't' || v[i - 1] != 't' && v[i + 1] == 'j' {
            v[i] = if "mqdzj".contains(v[i]) {
                v[i]
            } else {
                *d.get(&v[i]).unwrap_or(&v[i])
            };
        }
    }
    let x = v.iter().map(|k| m.get(k).unwrap_or(&0)).sum::<i32>();
    match x.signum() {
        -1 => "Left side wins!",
        0 => "Let's fight again!",
        _ => "Right side wins!",
    }
}
