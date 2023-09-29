use regex::Regex;

fn swap_adjacent_bits(n: u32) -> u32 {
    let mut s = format!("{n:032b}");
    s = Regex::new(r"(.)(.)").unwrap().replace_all(&s,"$2$1").to_string();
    u32::from_str_radix(&s,2).unwrap()
}