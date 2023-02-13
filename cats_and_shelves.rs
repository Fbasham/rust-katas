pub fn cats_and_shelves(x: u8, y: u8) -> u8 {
    (y - x) / 3 + (if (y - x) % 3 == 2 { 1 } else { 0 }) + (if (y - x) % 3 == 0 { 0 } else { 1 })
}
