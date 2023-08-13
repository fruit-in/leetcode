impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![vec![vec![0; m as usize + 1]; k as usize + 1]; n as usize + 1];
        dp[0][0][0] = 1;

        for a in 0..dp.len() - 1 {
            for b in 0..dp[0].len() {
                for c in 0..dp[0][0].len() {
                    dp[a + 1][b][c] = (dp[a + 1][b][c] + dp[a][b][c] * c as i64) % MOD;
                    if b < dp[0].len() - 1 {
                        for d in c + 1..dp[0][0].len() {
                            dp[a + 1][b + 1][d] = (dp[a + 1][b + 1][d] + dp[a][b][c]) % MOD;
                        }
                    }
                }
            }
        }

        dp[n as usize][k as usize]
            .iter()
            .fold(0, |acc, x| (acc + x) % MOD) as i32
    }
}
