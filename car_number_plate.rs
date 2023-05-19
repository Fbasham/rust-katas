fn find_the_number_plate(n: u32) -> String {
    let d = n % 999 + 1;
    let mut x = n / 999;
    let mut s = String::new();
    while x > 0 {
        s += &format!("{}", char::from_u32(x % 26 + 97).unwrap());
        x /= 26;
    }
    format!("{s:a<3}{d:03}")
}
