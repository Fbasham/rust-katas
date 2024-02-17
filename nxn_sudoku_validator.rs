use itertools::Itertools;

struct Sudoku {
    data: Vec<Vec<u32>>,
}

impl Sudoku {
    fn is_valid(&self) -> bool {
        let n = self.data.len();
        let m = self.data[0].len();
        let s = (n as f64).sqrt();
        if s % 1.0 != 0.0 {
            return false;
        }
        if self.data.iter().any(|e| e.iter().unique().count() != m) {
            return false;
        }
        if (0..m).any(|e| self.data.iter().map(|t| t[e]).unique().count() != n) {
            return false;
        }
        if self
            .data
            .iter()
            .any(|t| t.len() < 1 || t.iter().any(|e| e < &1 || e > &(n as u32)))
        {
            return false;
        }
        let k = s as usize;
        for i in (0..n).step_by(k) {
            for j in (0..m).step_by(k) {
                if &self.data[i..i + k]
                    .iter()
                    .map(|t| &t[j..j + k])
                    .flatten()
                    .unique()
                    .count()
                    != &n
                {
                    return false;
                }
            }
        }
        true
    }
}
