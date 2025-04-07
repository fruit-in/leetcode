use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut deque = VecDeque::from([(0, 0, true, 0)]);
        let mut visited = HashSet::from([(0, 0, true)]);

        while let Some((r, c, horizontal, step)) = deque.pop_front() {
            if r == n - 1 && c == n - 2 && horizontal {
                return step;
            }

            if horizontal {
                if c < n - 2 && grid[r][c + 2] == 0 && !visited.contains(&(r, c + 1, true)) {
                    deque.push_back((r, c + 1, true, step + 1));
                    visited.insert((r, c + 1, true));
                }
                if r < n - 1
                    && grid[r + 1][c] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r + 1, c, true))
                {
                    deque.push_back((r + 1, c, true, step + 1));
                    visited.insert((r + 1, c, true));
                }
                if r < n - 1
                    && grid[r + 1][c] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c, false))
                {
                    deque.push_back((r, c, false, step + 1));
                    visited.insert((r, c, false));
                }
            } else {
                if c < n - 1
                    && grid[r][c + 1] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c + 1, false))
                {
                    deque.push_back((r, c + 1, false, step + 1));
                    visited.insert((r, c + 1, false));
                }
                if r < n - 2 && grid[r + 2][c] == 0 && !visited.contains(&(r + 1, c, false)) {
                    deque.push_back((r + 1, c, false, step + 1));
                    visited.insert((r + 1, c, false));
                }
                if c < n - 1
                    && grid[r][c + 1] == 0
                    && grid[r + 1][c + 1] == 0
                    && !visited.contains(&(r, c, true))
                {
                    deque.push_back((r, c, true, step + 1));
                    visited.insert((r, c, true));
                }
            }
        }

        -1
    }
}
