impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = piles.len();
        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 0..n {
            for j in 0..=k {
                let mut total = 0;

                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                for m in 0..piles[i].len().min(k - j) {
                    total += piles[i][m];
                    dp[i + 1][j + m + 1] = dp[i + 1][j + m + 1].max(total + dp[i][j]);
                }
            }
        }

        dp[n][k]
    }
}
