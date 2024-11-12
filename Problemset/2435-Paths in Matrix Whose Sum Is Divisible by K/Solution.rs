use std::collections::VecDeque;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let mut prev_row_mod = vec![VecDeque::from(vec![0; k]); n];
        prev_row_mod[0][0] = 1;

        for r in 0..m {
            let mut row_mod = prev_row_mod.clone();
            row_mod[0].rotate_right(grid[r][0] as usize % k);

            for c in 1..n {
                for i in 0..k {
                    row_mod[c][i] = (row_mod[c][i] + row_mod[c - 1][i]) % 1_000_000_007;
                }
                row_mod[c].rotate_right(grid[r][c] as usize % k);
            }

            prev_row_mod = row_mod;
        }

        prev_row_mod[n - 1][0]
    }
}
