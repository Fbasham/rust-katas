mod preloaded;
use preloaded::{Direction, Position};

fn super_street_fighter_selection(a: &[&[&str]], p: Position, d: &[Direction]) -> Vec<String> {
    let mut v = vec![];
    let mut y = p.y as i32;
    let mut x = p.x as i32;
    let n = a.len() as i32;
    let m = a[0].len() as i32;
    for k in d {
        match k {
            Direction::Left | Direction::Right => {
                x = (x + (if k == &Direction::Left { -1 } else { 1 }) + m) % m;
                while a[y as usize][x as usize] == "" {
                    x = (x + (if k == &Direction::Left { -1 } else { 1 }) + m) % m;
                }
                v.push(a[y as usize][x as usize].to_string());
            }
            Direction::Up if y != 0 && a[(y - 1) as usize][x as usize] != "" => {
                y -= 1;
                v.push(a[y as usize][x as usize].to_string());
            }
            Direction::Down if y != n - 1 && a[(y + 1) as usize][x as usize] != "" => {
                y += 1;
                v.push(a[y as usize][x as usize].to_string());
            }
            _ => {
                v.push(a[y as usize][x as usize].to_string());
            }
        }
    }
    v
}
