use itertools::Itertools;

fn find_lowest_int(k: u64) -> u64 {
    for n in 1.. {
        let x = k * n;
        let y = (k + 1) * n;
        if x.to_string().chars().sorted().collect::<Vec<_>>()
            == y.to_string().chars().sorted().collect::<Vec<_>>()
        {
            return n;
        }
    }
    0
}
