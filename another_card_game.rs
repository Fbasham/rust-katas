use itertools::Itertools;

pub fn the_game(frank: &[u8; 4], sam: &[u8; 4], tom: &[u8; 4]) -> bool {
    let f = frank.iter().sorted().rev().collect::<Vec<_>>();
    let s = sam.iter().sorted().collect::<Vec<_>>();
    let t = tom.iter().sorted().collect::<Vec<_>>();
    f[0] > s[0] && f[0] > t[0] && f[1] > s[1] && f[1] > t[1]
        || f[1] > s[0] && f[1] > t[0] && f[0] > s[1] && f[0] > t[1]
}
