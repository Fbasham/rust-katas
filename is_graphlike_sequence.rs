use std::cmp::Reverse;

// Havel-Hakimi algorithm

pub fn solution(a: Vec<usize>) -> bool {
    let mut v: Vec<i32> = a.iter().cloned().map(|e| e as i32).collect();
    while a.len() > 0 {
        v.sort_by_key(|&e| Reverse(e));
        let x = v.remove(0);
        if (v.len() as i32) < x {
            return false;
        }
        for i in 0..x as usize {
            v[i] -= 1
        }
        if v.iter().all(|e| e == &0) {
            return true;
        }
        if v.iter().any(|e| e < &0) {
            return false;
        }
    }
    true
}
