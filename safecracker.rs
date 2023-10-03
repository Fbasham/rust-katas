fn safecracker(s: u8, t: &(u16, u16, u16)) -> (u8, u8, u8) {
    let x = (s as i32 - t.0 as i32 % 100 + 100) % 100;
    let y = (x + t.1 as i32 % 100) % 100;
    let z = (y - t.2 as i32 % 100 + 100) % 100;
    (x as u8, y as u8, z as u8)
}
