fn king_in_check(a: &[[char; 8]; 8]) -> bool {
    let ky = a.iter().position(|t| t.contains(&'♔')).unwrap();
    let kx = a[ky].iter().position(|e| e == &'♔').unwrap();
    for (dy, dx, c) in [
        (-1, -1, '♟'),
        (-1, 1, '♟'),
        (-2, -1, '♞'),
        (-1, -2, '♞'),
        (-2, 1, '♞'),
        (-1, 2, '♞'),
        (2, -1, '♞'),
        (1, -2, '♞'),
        (1, 2, '♞'),
        (2, 1, '♞'),
    ] {
        let y = ky as i32 + dy;
        let x = kx as i32 + dx;
        if y >= 0 && y < 8 && x >= 0 && x < 8 && a[y as usize][x as usize] == c {
            return true;
        }
    }
    for y in 0..8 {
        for x in 0..8 {
            if "♛♝♜".contains(a[y][x]) {
                if (y == ky || x == kx) && a[y][x] != '♝' {
                    let mut f = false;
                    for i in 1..(y as i32 - ky as i32).abs() {
                        if !"♔ "
                            .contains(a[(y as i32 + (if y < ky { 1 } else { -1 }) * i) as usize][x])
                        {
                            f = true;
                        }
                    }
                    for i in 1..(x as i32 - kx as i32).abs() {
                        if !"♔ "
                            .contains(a[y][(x as i32 + (if x < kx { 1 } else { -1 }) * i) as usize])
                        {
                            f = true;
                        }
                    }
                    if f == false {
                        return true;
                    }
                }
                if (ky as i32 - y as i32).abs() == (kx as i32 - x as i32).abs() && a[y][x] != '♜'
                {
                    let mut f = false;
                    for i in 1..(y as i32 - ky as i32).abs() {
                        if !"♔ ".contains(
                            a[(y as i32 + (if y < ky { 1 } else { -1 }) * i) as usize]
                                [(x as i32 + (if x < kx { 1 } else { -1 }) * i) as usize],
                        ) {
                            f = true;
                        }
                    }
                    if f == false {
                        return true;
                    }
                }
            }
        }
    }
    false
}
