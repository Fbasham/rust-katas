mod kata_solution {
    use itertools::Itertools;
    pub fn mystery_range(s: &str, n: usize) -> (u32, u32) {
        let h = s.chars().counts();
        for i in 1..100 {
            let d = (i..i + n).map(|e| e.to_string()).join("").chars().counts();
            if d == h {
                return (i as u32, i as u32 + n as u32 - 1);
            }
        }
        (0, 0)
    }
}
