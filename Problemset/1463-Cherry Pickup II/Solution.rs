impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut curr_row = vec![vec![None; cols]; cols];
        curr_row[0][cols - 1] = Some(grid[0][0] + grid[0][cols - 1]);

        for i in 0..rows - 1 {
            let mut next_row = vec![vec![None; cols]; cols];

            for j0 in 0..cols {
                for j1 in 0..cols {
                    if let Some(x) = curr_row[j0][j1] {
                        for c0 in j0.saturating_sub(1)..cols.min(j0 + 2) {
                            for c1 in j1.saturating_sub(1)..cols.min(j1 + 2) {
                                next_row[c0][c1] = Some(next_row[c0][c1].unwrap_or(0).max(
                                    x + grid[i + 1][c0] + grid[i + 1][c1] * (c0 != c1) as i32,
                                ));
                            }
                        }
                    }
                }
            }

            curr_row = next_row;
        }

        curr_row
            .iter()
            .flatten()
            .map(|x| x.unwrap_or(0))
            .max()
            .unwrap()
    }
}
