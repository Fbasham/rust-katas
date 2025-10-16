fn f(str1: &str, str2: &str) -> usize {
    let m = str1.len();
    let n = str2.len();
    let mut dp = (0..m + 1)
        .map(|_| (0..n + 1).map(|_| 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for i in 0..m + 1 {
        dp[i][0] = i;
    }
    for j in 0..n + 1 {
        dp[0][j] = j
    }
    for i in 1..m + 1 {
        for j in 1..n + 1 {
            if str1.chars().nth(i - 1).unwrap() == str2.chars().nth(j - 1).unwrap() {
                dp[i][j] = dp[i - 1][j - 1]
            } else {
                dp[i][j] = 1 + dp[i][j - 1].min(dp[i - 1][j].min(dp[i - 1][j - 1]))
            }
        }
    }
    return dp[m][n];
}

struct Dictionary<'a> {
    words: &'a [&'a str],
}

impl<'a> Dictionary<'a> {
    fn new(words: &'a [&str]) -> Self {
        Self { words }
    }

    fn find_most_similar(&self, t: &str) -> &str {
        self.words.iter().min_by_key(|s| f(s, t)).unwrap()
    }
}
