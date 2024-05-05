mod ziggurat {
    pub fn ride_of_fortune(artifact: &[&str], explorers: &[usize]) -> Vec<Option<(usize, usize)>> {
        let mut a = artifact
            .iter()
            .map(|e| e.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut r = vec![];
        for i in 0..explorers.len() {
            let mut dy: i32 = 0;
            let mut dx: i32 = 1;
            let mut y = explorers[i] as i32;
            let mut x: i32 = 0;
            loop {
                if a[y as usize][x as usize] == 'A' {
                    if dx == 1 {
                        dy = 1;
                        dx = 0;
                    } else if dx == -1 {
                        dy = -1;
                        dx = 0;
                    } else if dy == 1 {
                        dy = 0;
                        dx = 1;
                    } else if dy == -1 {
                        dy = 0;
                        dx = -1;
                    }
                    a[y as usize][x as usize] = 'B';
                } else if a[y as usize][x as usize] == 'B' {
                    if dx == 1 {
                        dy = -1;
                        dx = 0;
                    } else if dx == -1 {
                        dy = 1;
                        dx = 0;
                    } else if dy == 1 {
                        dy = 0;
                        dx = -1;
                    } else if dy == -1 {
                        dy = 0;
                        dx = 1;
                    }
                    a[y as usize][x as usize] = 'A';
                }
                y = y + dy;
                x = x + dx;
                if x < 0 && dx == -1 {
                    r.push(None);
                    break;
                }
                if (x > a[0].len() as i32 - 1 && dx == 1)
                    || (y < 0 && dy == -1)
                    || (y > a.len() as i32 - 1 && dy == 1)
                {
                    r.push(Some(((y - dy) as usize, (x - dx) as usize)));
                    break;
                }
            }
        }
        r
    }
}
