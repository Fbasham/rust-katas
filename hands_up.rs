fn get_positions(n: u32) -> [u32; 3] {
    [n%3,(n%9)/3,(n%27)/9]
}