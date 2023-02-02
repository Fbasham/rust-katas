struct Block {
    width: u32,
    length: u32,
    height: u32,
}

impl Block {
    fn new(s: &[u32]) -> Block {
        Block {
            width: s[0],
            length: s[1],
            height: s[2],
        }
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_length(&self) -> u32 {
        self.length
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_volume(&self) -> u32 {
        self.width * self.length * self.height
    }
    fn get_surface_area(&self) -> u32 {
        2 * self.width * self.length + 2 * self.length * self.height + 2 * self.height * self.width
    }
}
