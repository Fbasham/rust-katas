use itertools::Itertools;

#[derive(Debug)]
#[derive(PartialEq,Eq,Hash)]
enum Resource {
    Sheep,
    Wood,
    Brick,
    Stone,
    Wheat
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Structure {
    City,
    Settlement,
    Road
}

fn can_build(hand: Vec<Resource>, p2_hand: Vec<Resource>, p3_hand: Vec<Resource>, target: Structure) -> bool {  
    let mut d = hand.iter().counts();
    let mut v = vec![];
    if target==Structure::Road {
        v = vec![0,1usize.saturating_sub(*d.get(&Resource::Wood).unwrap_or(&0)),1usize.saturating_sub(*d.get(&Resource::Brick).unwrap_or(&0)),0,0];
        d.insert(&Resource::Wood,d.get(&Resource::Wood).unwrap_or(&0).saturating_sub(1));
        d.insert(&Resource::Brick,d.get(&Resource::Brick).unwrap_or(&0).saturating_sub(1));
    }    
    if target==Structure::Settlement {
        v = vec![1usize.saturating_sub(*d.get(&Resource::Sheep).unwrap_or(&0)),1usize.saturating_sub(*d.get(&Resource::Wood).unwrap_or(&0)),1usize.saturating_sub(*d.get(&Resource::Brick).unwrap_or(&0)),0,1usize.saturating_sub(*d.get(&Resource::Wheat).unwrap_or(&0))];
        d.insert(&Resource::Sheep,d.get(&Resource::Sheep).unwrap_or(&0).saturating_sub(1));
        d.insert(&Resource::Wood,d.get(&Resource::Wood).unwrap_or(&0).saturating_sub(1));
        d.insert(&Resource::Brick,d.get(&Resource::Brick).unwrap_or(&0).saturating_sub(1));
        d.insert(&Resource::Wheat,d.get(&Resource::Wheat).unwrap_or(&0).saturating_sub(1));
    }    
    if target==Structure::City {
        v = vec![0,0,0,2usize.saturating_sub(*d.get(&Resource::Stone).unwrap_or(&0)),3usize.saturating_sub(*d.get(&Resource::Wheat).unwrap_or(&0))];
        d.insert(&Resource::Stone,d.get(&Resource::Stone).unwrap_or(&0).saturating_sub(2));
        d.insert(&Resource::Wheat,d.get(&Resource::Wheat).unwrap_or(&0).saturating_sub(3));
    }
    let n = d.values().sum::<usize>();
    let dd = p2_hand.iter().chain(p3_hand.iter()).counts();
    dd.get(&Resource::Sheep).unwrap_or(&0)>=&v[0] &&
    dd.get(&Resource::Wood).unwrap_or(&0)>=&v[1] &&
    dd.get(&Resource::Brick).unwrap_or(&0)>=&v[2] &&
    dd.get(&Resource::Stone).unwrap_or(&0)>=&v[3] &&
    dd.get(&Resource::Wheat).unwrap_or(&0)>=&v[4]
}