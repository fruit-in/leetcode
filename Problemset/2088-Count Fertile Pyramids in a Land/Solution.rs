impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut ret = 0;

        for i in 1..m {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    continue;
                }

                dp[i][j] = grid[i - 1][j - 1] * grid[i - 1][j] * grid[i - 1][j + 1];
                dp[i][j] += dp[i][j] * dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j + 1]);
                ret += dp[i][j];
            }
        }

        dp = vec![vec![0; n]; m];

        for i in (0..m - 1).rev() {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    continue;
                }

                dp[i][j] = grid[i + 1][j - 1] * grid[i + 1][j] * grid[i + 1][j + 1];
                dp[i][j] += dp[i][j] * dp[i + 1][j - 1].min(dp[i + 1][j]).min(dp[i + 1][j + 1]);
                ret += dp[i][j];
            }
        }

        ret
    }
}
