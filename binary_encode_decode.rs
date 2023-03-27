use std::collections::HashMap;

fn code(s: &str) -> String {
    let d = HashMap::from([
        ('0', "10"),
        ('1', "11"),
        ('2', "0110"),
        ('3', "0111"),
        ('4', "001100"),
        ('5', "001101"),
        ('6', "001110"),
        ('7', "001111"),
        ('8', "00011000"),
        ('9', "00011001"),
    ]);
   s.chars().map(|k| d[&k]).collect()
}

fn decode(s: &str) -> String {
    let d = HashMap::from([
        ("10", "0"),
        ("11", "1"),
        ("0110", "2"),
        ("0111", "3"),
        ("001100", "4"),
        ("001101", "5"),
        ("001110", "6"),
        ("001111", "7"),
        ("00011000", "8"),
        ("00011001", "9"),
    ]);
    let mut r = String::new();
    let mut t = s.to_string();
    while t.len()>0 {
        for (k,v) in d.iter() {
            if t.starts_with(k){
                r += v;
                t = t[k.len()..].to_string();
                break;
            }
        }
    }
    r
}