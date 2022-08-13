impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let row_len = grid.len();
        let col_len = grid[0].len();
        let mut prefix_sum_row = vec![vec![0; col_len]; row_len];
        let mut prefix_sum_col = vec![vec![0; col_len]; row_len];
        let mut max_len = 0;

        for row in 0..row_len {
            for col in 0..col_len {
                if grid[row][col] == 0 {
                    continue;
                }

                if col > 0 {
                    prefix_sum_row[row][col] = prefix_sum_row[row][col - 1];
                }
                if row > 0 {
                    prefix_sum_col[row][col] = prefix_sum_col[row - 1][col];
                }
                prefix_sum_row[row][col] += 1;
                prefix_sum_col[row][col] += 1;

                for len in
                    (max_len + 1..=prefix_sum_row[row][col].min(prefix_sum_col[row][col])).rev()
                {
                    if prefix_sum_row[row + 1 - len][col] >= len
                        && prefix_sum_col[row][col + 1 - len] >= len
                    {
                        max_len = len;
                        break;
                    }
                }
            }
        }

        (max_len * max_len) as i32
    }
}
