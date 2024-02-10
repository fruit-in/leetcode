impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![0_i32; k + 1]; n + 1];

        dp[1][0] = 1;

        for i in 2..=n {
            for j in 0..=k {
                dp[i][j] = dp[i - 1][j];
                if j > 0 {
                    dp[i][j] = (dp[i][j] + dp[i][j - 1]) % 1_000_000_007;
                }
                if j >= i {
                    dp[i][j] = (dp[i][j] - dp[i - 1][j - i]).rem_euclid(1_000_000_007);
                }
            }
        }

        dp[n][k]
    }
}
