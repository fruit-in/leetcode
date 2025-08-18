use std::collections::VecDeque;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let (m, n) = (grid.len(), grid[0].len());

        for i in 0..m.min(n) / 2 {
            let mut indices = vec![];
            let mut vals = VecDeque::new();

            for r in i..m - i {
                indices.push([r, i]);
                vals.push_back(grid[r][i]);
            }
            for c in i + 1..n - i {
                indices.push([m - 1 - i, c]);
                vals.push_back(grid[m - 1 - i][c]);
            }
            for r in (i..m - 1 - i).rev() {
                indices.push([r, n - 1 - i]);
                vals.push_back(grid[r][n - 1 - i]);
            }
            for c in (i + 1..n - 1 - i).rev() {
                indices.push([i, c]);
                vals.push_back(grid[i][c]);
            }

            vals.rotate_right(k % vals.len());
            for j in 0..vals.len() {
                grid[indices[j][0]][indices[j][1]] = vals[j];
            }
        }

        grid
    }
}
