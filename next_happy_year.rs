use itertools::Itertools;

fn next_happy_year(year: u16) -> u16 {
    let mut n = year + 1;
    loop {
        if n.to_string().chars().unique().count() == n.to_string().chars().count() {
            return n;
        }
        n += 1
    }
}
