impl Solution {
    pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 {
            return grid[0][0];
        }

        let mut min_col2 = [grid.len(); 2];
        let mut min_sum2 = [0; 2];

        for r in 0..grid.len() {
            let mut i = grid.len();
            let mut j = grid.len();

            for c in 0..grid.len() {
                if c != min_col2[0] {
                    grid[r][c] += min_sum2[0];
                } else {
                    grid[r][c] += min_sum2[1];
                }

                if grid[r][c] <= *grid[r].get(i).unwrap_or(&i32::MAX) {
                    j = i;
                    i = c;
                } else if grid[r][c] < *grid[r].get(j).unwrap_or(&i32::MAX) {
                    j = c;
                }
            }

            min_col2 = [i, j];
            min_sum2 = [grid[r][i], grid[r][j]];
        }

        min_sum2[0].min(min_sum2[1])
    }
}
