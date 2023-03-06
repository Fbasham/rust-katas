#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn dir_reduc(a: &[Direction]) -> Vec<Direction> {
    let mut v = vec![];
    for d in a {
        let p = *v.clone().last().unwrap_or(&Direction::North);
        if v.is_empty() {
            v.push(d.clone())
        } else if d == &Direction::North && p == Direction::South
            || d == &Direction::South && p == Direction::North
            || d == &Direction::West && p == Direction::East
            || d == &Direction::East && p == Direction::West
        {
            v.pop();
        } else {
            v.push(d.clone())
        }
    }
    v
}
