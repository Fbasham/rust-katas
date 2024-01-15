mod preloaded;

use preloaded::{Resource, Structure};

fn can_build(
    hand: Vec<Resource>,
    p2_hand: Vec<Resource>,
    p3_hand: Vec<Resource>,
    target: Structure,
) -> bool {
    let mut d = vec![0, 0, 0, 0, 0];
    let mut dd = vec![0, 0, 0, 0, 0];
    for e in hand {
        match e {
            Resource::Sheep => d[0] += 1,
            Resource::Wood => d[1] += 1,
            Resource::Brick => d[2] += 1,
            Resource::Stone => d[3] += 1,
            Resource::Wheat => d[4] += 1,
        }
    }
    for e in p2_hand.iter().chain(p3_hand.iter()) {
        match e {
            &Resource::Sheep => dd[0] += 1,
            &Resource::Wood => dd[1] += 1,
            &Resource::Brick => dd[2] += 1,
            &Resource::Stone => dd[3] += 1,
            &Resource::Wheat => dd[4] += 1,
        }
    }
    let mut v = vec![];
    match target {
        Structure::Road => {
            v = vec![
                0,
                1usize.saturating_sub(d[1]),
                1usize.saturating_sub(d[2]),
                0,
                0,
            ];
            d[1] = d[1].saturating_sub(1);
            d[2] = d[2].saturating_sub(1);
        }
        Structure::Settlement => {
            v = vec![
                1usize.saturating_sub(d[0]),
                1usize.saturating_sub(d[1]),
                1usize.saturating_sub(d[2]),
                0,
                1usize.saturating_sub(d[4]),
            ];
            d[0] = d[0].saturating_sub(1);
            d[1] = d[1].saturating_sub(1);
            d[2] = d[2].saturating_sub(1);
            d[4] = d[4].saturating_sub(1);
        }
        Structure::City => {
            v = vec![
                0,
                0,
                0,
                2usize.saturating_sub(d[3]),
                3usize.saturating_sub(d[4]),
            ];
            d[3] = d[3].saturating_sub(2);
            d[4] = d[4].saturating_sub(3);
        }
    }
    dd.iter().enumerate().all(|(i, e)| e >= &v[i])
        && d.iter().sum::<usize>() >= v.iter().sum::<usize>()
}
