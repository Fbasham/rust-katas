use itertools::Itertools;

fn choose_best_sum(t: i32, k: i32, v: &Vec<i32>) -> i32 {
    v.into_iter()
        .combinations(k as usize)
        .map(|c| c.into_iter().sum::<i32>())
        .filter(|&e| e <= t)
        .min_by_key(|e| t - e)
        .unwrap_or(-1)
}
