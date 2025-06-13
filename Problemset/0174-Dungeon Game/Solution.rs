impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![vec![0; n]; m];

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if r < m - 1 && c < n - 1 {
                    dp[r][c] = 1.max(dp[r + 1][c].min(dp[r][c + 1]) - dungeon[r][c]);
                } else if r < m - 1 {
                    dp[r][c] = 1.max(dp[r + 1][c] - dungeon[r][c]);
                } else if c < n - 1 {
                    dp[r][c] = 1.max(dp[r][c + 1] - dungeon[r][c]);
                } else {
                    dp[r][c] = 1.max(1 - dungeon[r][c]);
                }
            }
        }

        dp[0][0]
    }
}
