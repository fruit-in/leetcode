impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut dp = vec![poured as f64];

        for r in 0..query_row as usize {
            dp.push((dp[r] - 1.0).max(0.0) / 2.0);
            for i in (1..=r).rev() {
                dp[i] = (dp[i] - 1.0).max(0.0) / 2.0 + (dp[i - 1] - 1.0).max(0.0) / 2.0;
            }
            dp[0] = (dp[0] - 1.0).max(0.0) / 2.0;
        }

        dp[query_glass as usize].min(1.0)
    }
}
