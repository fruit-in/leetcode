impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![[[0_i64; 3]; 2]; n + 1];
        let mut ret = 0;
        dp[0][0][0] = 1;

        for i in 0..n {
            dp[i + 1][1][0] = dp[i][0][0] + dp[i][0][1] + dp[i][0][2];
            for j in 0..2 {
                dp[i + 1][j][0] += dp[i][j][0] + dp[i][j][1] + dp[i][j][2];
                dp[i + 1][j][0] %= 1_000_000_007;
                for k in 0..2 {
                    dp[i + 1][j][k + 1] += dp[i][j][k];
                    dp[i + 1][j][k + 1] %= 1_000_000_007;
                }
            }
        }

        for j in 0..2 {
            for k in 0..3 {
                ret = (ret + dp[n][j][k] as i32) % 1_000_000_007;
            }
        }

        ret
    }
}
