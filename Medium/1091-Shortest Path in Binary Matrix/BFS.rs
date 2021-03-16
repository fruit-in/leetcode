use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut seen = vec![(0, 0)].into_iter().collect::<HashSet<_>>();
        let mut cells = vec![(0, 0, 1)].into_iter().collect::<VecDeque<_>>();

        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }

        while let Some((x, y, length)) = cells.pop_front() {
            if x == n - 1 && y == n - 1 {
                return length;
            }
            for x_ in x.saturating_sub(1)..(x + 2).min(n) {
                for y_ in y.saturating_sub(1)..(y + 2).min(n) {
                    if grid[x_][y_] == 0 && !seen.contains(&(x_, y_)) {
                        seen.insert((x_, y_));
                        cells.push_back((x_, y_, length + 1));
                    }
                }
            }
        }

        -1
    }
}
