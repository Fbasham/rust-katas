fn can_play(hand: &[&str], face_up: &str) -> bool {
    hand.iter()
        .any(|s| s.split(" ").zip(face_up.split(" ")).any(|(x, y)| x == y))
}
