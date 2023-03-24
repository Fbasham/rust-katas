mod preloaded;
use preloaded::Direction;

fn street_fighter_selection(
    fighters: &[[&str; 6]; 2],
    position: &[i64; 2],
    moves: &[Direction],
) -> Vec<String> {
    let mut i = position[0];
    let mut j = position[1];
    let n = fighters.len() as i64;
    let m = fighters[0].len() as i64;
    let mut v = vec![];
    for d in moves {
        match d {
            Direction::Down => (i,j) = (i+1,j),
            Direction::Up => (i,j) = (i-1,j),
            Direction::Left => (i,j) = (i,j-1),
            _ => (i,j) = (i,j+1),
        }
        i = if i<0 {0} else if i>=n {n-1} else {i};
        j = if j<0 {m-1} else if j>=m {0} else {j};
        v.push(fighters[i as usize][j as usize].to_string())
    }
    v
}