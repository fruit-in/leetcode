impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        dp[m - 1][n - 1] = 1 - obstacle_grid[m - 1][n - 1];

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if obstacle_grid[i][j] == 0 {
                    if i < m - 1 {
                        dp[i][j] += dp[i + 1][j];
                    }
                    if j < n - 1 {
                        dp[i][j] += dp[i][j + 1];
                    }
                }
            }
        }

        dp[0][0]
    }
}
