use std::collections::VecDeque;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut removes = vec![vec![(m * n) as i32; n]; m];
        let mut cells = VecDeque::from([(0, 0)]);
        removes[0][0] = 0;

        while let Some((r0, c0)) = cells.pop_front() {
            for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let r1 = (r0 as i32 + x) as usize;
                let c1 = (c0 as i32 + y) as usize;

                if r1 < m && c1 < n && removes[r1][c1] > removes[r0][c0] + grid[r1][c1] {
                    removes[r1][c1] = removes[r0][c0] + grid[r1][c1];
                    if grid[r1][c1] == 0 {
                        cells.push_front((r1, c1));
                    } else {
                        cells.push_back((r1, c1));
                    }
                }
            }
        }

        removes[m - 1][n - 1]
    }
}
