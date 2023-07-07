use std::collections::HashSet;

fn validate_sudoku(a: &[[u8; 9]; 9]) -> bool {
    let s = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    if a.iter().cloned().any(|t| HashSet::from(t) != s) {
        return false;
    }
    if (0..9).any(|i| HashSet::from_iter(a.iter().cloned().map(|t| t[i])) != s) {
        return false;
    }
    for i in (0..9).step_by(3) {
        if HashSet::from_iter(
            a[i..i + 3]
                .iter()
                .map(|e| &e[i..i + 3])
                .flatten()
                .into_iter()
                .cloned(),
        ) != s
        {
            return false;
        }
    }
    true
}
