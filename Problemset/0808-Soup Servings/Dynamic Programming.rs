impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let n = (n as usize + 24) / 25;
        if n > 222 {
            return 1_f64;
        }

        let mut dp = vec![vec![0_f64; n + 1]; n + 1];
        dp[0] = vec![1_f64; n + 1];
        dp[0][0] = 0.5;

        for i in 1..=n {
            for j in 1..=n {
                dp[i][j] += dp[i.max(4) - 4][j];
                dp[i][j] += dp[i.max(3) - 3][j - 1];
                dp[i][j] += dp[i.max(2) - 2][j.max(2) - 2];
                dp[i][j] += dp[i - 1][j.max(3) - 3];
                dp[i][j] *= 0.25;
            }
        }

        dp[n][n]
    }
}
