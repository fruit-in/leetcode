impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if word1[i] == word2[j] {
                    if i > 0 && j > 0 {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                    dp[i][j] += 1;
                }
                if i > 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1]);
                }
            }
        }

        (m + n - 2 * dp[m - 1][n - 1]) as i32
    }
}
