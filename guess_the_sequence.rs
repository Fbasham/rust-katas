use itertools::Itertools;

fn sequence(x: u8) -> Vec<u8> {
    (1..x + 1).sorted_by_key(|e| e.to_string()).collect()
}
