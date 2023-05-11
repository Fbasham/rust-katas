fn number_of_carries(na: u32, nb: u32) -> usize {
    let mut a = na.to_string();
    let mut b = nb.to_string();
    let m = a.len().max(b.len());
    a = "0".repeat(m - a.len()) + &a;
    b = "0".repeat(m - b.len()) + &b;
    let mut c = 0;
    let mut t = 0;
    for i in (0..m).rev() {
        let x = a.chars().nth(i).unwrap().to_digit(10).unwrap()
            + b.chars().nth(i).unwrap().to_digit(10).unwrap()
            + t;
        t = x / 10;
        c += x / 10;
    }
    c as usize
}
