fn f32_to_ieee_754(mut n: f32) -> String {
    if n == 0.0 && n.is_sign_negative() {
        return "1 00000000 00000000000000000000000".to_string();
    }
    if n == 0.0 {
        return "0 00000000 00000000000000000000000".to_string();
    }
    if n.is_infinite() && n.is_sign_negative() {
        return "1 11111111 00000000000000000000000".to_string();
    }
    if n.is_infinite() {
        return "0 11111111 00000000000000000000000".to_string();
    }
    if n.is_nan() && n.is_sign_negative() {
        return "1 11111111 10000000000000000000000".to_string();
    }
    if n.is_nan() {
        return "0 11111111 10000000000000000000000".to_string();
    }
    let is_neg = if n < 0.0 { 1 } else { 0 };
    n = n.abs();
    let r = format!("{:b}", n as u32);
    let mut t = String::new();
    n %= 1.0;
    while n != 0.0 {
        t += if 2.0 * n < 1.0 { "0" } else { "1" };
        n = (2.0 * n) % 1.0;
    }
    let s = format!("{r}{t}");
    let i = s.chars().position(|e| e == '1').unwrap();
    let e = 127
        + (if i == 0 {
            r.len() as i32 - 1
        } else {
            -1 * i as i32
        });
    let x = 23_usize.saturating_sub(s[i + 1..].len());
    format!("{is_neg} {e:08b} {}{}", &s[i + 1..], "0".repeat(x))
}

fn f64_to_ieee_754(mut n: f64) -> String {
    if n == 0.0 && n.is_sign_negative() {
        return "1 00000000000 0000000000000000000000000000000000000000000000000000".to_string();
    }
    if n == 0.0 {
        return "0 00000000000 0000000000000000000000000000000000000000000000000000".to_string();
    }
    if n.is_infinite() && n.is_sign_negative() {
        return "1 11111111111 0000000000000000000000000000000000000000000000000000".to_string();
    }
    if n.is_infinite() {
        return "0 11111111111 0000000000000000000000000000000000000000000000000000".to_string();
    }
    if n.is_nan() && n.is_sign_negative() {
        return "1 11111111111 1000000000000000000000000000000000000000000000000000".to_string();
    }
    if n.is_nan() {
        return "0 11111111111 1000000000000000000000000000000000000000000000000000".to_string();
    }
    let is_neg = if n < 0.0 { 1 } else { 0 };
    n = n.abs();
    let r = format!("{:b}", n as u32);
    let mut t = String::new();
    n %= 1.0;
    while n != 0.0 {
        t += if 2.0 * n < 1.0 { "0" } else { "1" };
        n = (2.0 * n) % 1.0;
    }
    let s = format!("{r}{t}");
    let i = s.chars().position(|e| e == '1').unwrap();
    let e = 1023
        + (if i == 0 {
            r.len() as i32 - 1
        } else {
            -1 * i as i32
        });
    let x = 52_usize.saturating_sub(s[i + 1..].len());
    format!("{is_neg} {e:011b} {}{}", &s[i + 1..], "0".repeat(x))
}
