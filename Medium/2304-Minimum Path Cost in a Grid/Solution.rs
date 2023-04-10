impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp0 = grid[0].clone();
        let mut dp1 = vec![i32::MAX; n];

        for x in 0..m - 1 {
            for y in 0..n {
                let i = grid[x][y] as usize;

                for j in 0..n {
                    dp1[j] = dp1[j].min(dp0[y] + move_cost[i][j] + grid[x + 1][j]);
                }
            }

            dp0 = dp1;
            dp1 = vec![i32::MAX; n];
        }

        *dp0.iter().min().unwrap()
    }
}
