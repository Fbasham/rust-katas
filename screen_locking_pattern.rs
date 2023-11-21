use std::collections::HashMap;

fn count_patterns(s: char, n: u8) -> u64 {
    if n < 1 || n > 9 {
        return 0;
    }
    let d = HashMap::from([
        ('A', "BFEHD"),
        ('B', "ADGEIFC"),
        ('C', "BDEHF"),
        ('D', "ABCEIHG"),
        ('E', "ABCDFGHI"),
        ('F', "ABCEGHI"),
        ('G', "DBEFH"),
        ('H', "GDAECFI"),
        ('I', "HDEBF"),
    ]);
    let dd = HashMap::from([
        ('A', vec!["BC", "DG", "EI"]),
        ('B', vec!["EH"]),
        ('C', vec!["BA", "EG", "FI"]),
        ('D', vec!["EF"]),
        ('F', vec!["ED"]),
        ('G', vec!["DA", "EC", "HI"]),
        ('H', vec!["EB"]),
        ('I', vec!["HG", "EA", "FC"]),
    ]);
    let mut q = vec![(s, s.to_string())];
    let mut c = 0;
    while q.len() > 0 {
        let (k, r) = q.pop().unwrap();
        if r.len() == n as usize {
            c += 1;
            continue;
        }
        for e in d.get(&k).unwrap().chars() {
            if !r.contains(e) {
                q.push((e, format!("{r}{e}")))
            }
        }
        if k != 'E' {
            for e in dd.get(&k).unwrap() {
                let x = e.chars().nth(0).unwrap();
                let y = e.chars().nth(1).unwrap();
                if r.contains(x) && !r.contains(y) {
                    q.push((y, format!("{r}{y}")));
                }
            }
        }
    }
    c
}
