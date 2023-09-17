impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let steps = steps as usize;
        let arr_len = arr_len as usize;
        let mut dp = vec![vec![0; arr_len.min(steps / 2 + 1)]; steps + 1];
        dp[0][0] = 1;

        for i in 0..steps {
            for j in 0..dp[0].len() {
                dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % 1_000_000_007;
                if j > 0 {
                    dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % 1_000_000_007;
                }
                if j < dp[0].len() - 1 {
                    dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % 1_000_000_007;
                }
            }
        }

        dp[steps][0]
    }
}
