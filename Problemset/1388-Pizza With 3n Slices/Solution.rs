impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        let m = n / 3;
        let mut dp = vec![vec![0; m + 1]; n];

        dp[1][1] = slices[1];
        for i in 2..n {
            for j in 1..=m.min(i / 2 + 1) {
                dp[i][j] = dp[i - 1][j].max(dp[i - 2][j - 1] + slices[i]);
            }
        }

        dp[0][1] = slices[0];
        dp[1][1] = slices[0].max(slices[1]);
        for i in 2..n - 1 {
            for j in 1..=m.min(i / 2 + 1) {
                dp[i][j] = dp[i - 1][j].max(dp[i - 2][j - 1] + slices[i]);
            }
        }

        dp[n - 1][m].max(dp[n - 2][m])
    }
}
