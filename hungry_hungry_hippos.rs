struct Game {
    a: Vec<Vec<i32>>
}

impl Game {
    fn new(board: Vec<Vec<i32>>) -> Self {
        Game {a: board}
    }

    fn play(&mut self) -> usize {
        let mut v = vec![];
        for i in 0..self.a.len() {
            for j in 0..self.a[i].len() {
                if self.a[i][j]==1 {
                    v.push((i,j));
                }
            }
        }
        let mut c = 0;
        while v.len()>0 {
            let mut t = vec![v.pop().unwrap()];
            c += 1;
            while t.len()>0 {
                let (y,x) = t.pop().unwrap();
                if v.contains(&(y,x)) {
                    let k = v.iter().position(|t| t==&(y,x)).unwrap();
                    v.remove(k);
                }
                for (i,j) in [(y.saturating_sub(1),x),(y+1,x),(y,x.saturating_sub(1)),(y,x+1)] {
                    if v.contains(&(i,j)) {
                        t.push((i,j));
                    }
                }
            }
        }
        c
    }
}