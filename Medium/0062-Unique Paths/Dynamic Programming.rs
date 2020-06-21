impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];
        dp[m - 1][n - 1] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i < m - 1 {
                    dp[i][j] += dp[i + 1][j];
                }
                if j < n - 1 {
                    dp[i][j] += dp[i][j + 1];
                }
            }
        }

        dp[0][0]
    }
}
