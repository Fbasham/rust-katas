fn solution(n: i32) -> i32 {
    if (n < 0) {
        return 0;
    }
    (0..n).filter(|&e| e % 3 == 0 || e % 5 == 0).sum()
}
