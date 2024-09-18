fn cuckoo_clock(t: &str, n: u32) -> String {
    let mut h = t.split(":").nth(0).unwrap().parse::<i32>().unwrap();
    let mut m = t.split(":").nth(1).unwrap().parse::<i32>().unwrap();
    let mut k = n as i32;
    loop {
        if m % 15 == 0 {
            k -= if m == 0 {
                if h == 0 {
                    12
                } else {
                    h
                }
            } else {
                1
            };
        }
        if k <= 0 {
            break;
        }
        m += 1;
        if m >= 60 {
            h = (h + 1) % 12;
            m %= 60;
        }
    }
    format!("{:02}:{m:02}", if h == 0 { 12 } else { h })
}
