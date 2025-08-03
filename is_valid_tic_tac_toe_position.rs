use crate::preloaded::Player;
use itertools::Itertools;

fn is_valid_position(b: &[[Player; 3]; 3]) -> bool {
    let h = b
        .iter()
        .map(|t| {
            t.map(|e| match e {
                Player::None => "_",
                Player::X => "x",
                Player::O => "o",
            })
            .join("")
        })
        .collect::<Vec<_>>();
    let v = (0..3)
        .map(|i| h.iter().map(|t| t.chars().nth(i).unwrap()).join(""))
        .collect::<Vec<_>>();
    let d = vec![
        (0..3).map(|i| h[i].chars().nth(i).unwrap()).join(""),
        (0..3).map(|i| h[i].chars().nth(2 - i).unwrap()).join(""),
    ];
    let t = h.iter().chain(v.iter()).chain(d.iter()).collect::<Vec<_>>();
    let s = v.join("");
    let x = s.matches("x").count();
    let o = s.matches("o").count();
    if o > x || (x as i32 - o as i32).abs() > 1 {
        return false;
    }
    if t.iter().filter(|&&s| s == "xxx").count() > 0
        && t.iter().filter(|&&s| s == "ooo").count() > 0
    {
        return false;
    }
    if o == x && t.iter().filter(|&&s| s == "xxx").count() == 1 {
        return false;
    }
    if x > o && t.iter().filter(|&&s| s == "ooo").count() == 1 {
        return false;
    }
    true
}
