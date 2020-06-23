impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = grid.clone();

        for i in 0..m {
            for j in 0..n {
                dp[i][j] += match (i, j) {
                    (0, 0) => 0,
                    (0, _) => dp[i][j - 1],
                    (_, 0) => dp[i - 1][j],
                    _ => dp[i][j - 1].min(dp[i - 1][j]),
                };
            }
        }

        dp[m - 1][n - 1]
    }
}
