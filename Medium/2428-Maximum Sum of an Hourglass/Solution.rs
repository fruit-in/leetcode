impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                ret = ret.max(
                    grid[i][j]
                        + grid[i - 1][j]
                        + grid[i + 1][j]
                        + grid[i - 1][j - 1]
                        + grid[i + 1][j - 1]
                        + grid[i - 1][j + 1]
                        + grid[i + 1][j + 1],
                );
            }
        }

        ret
    }
}
