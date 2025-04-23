use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = HashMap::from([((0, 0), 0)]);
        let mut deque = VecDeque::from([(0, 0, 0, 0)]);

        while let Some((i, j, obstacles, steps)) = deque.pop_front() {
            if i == m - 1 && j == n - 1 {
                return steps;
            }

            if i > 0 {
                if grid[i - 1][j] == 1
                    && obstacles + 1 <= k
                    && *visited.get(&(i - 1, j)).unwrap_or(&i32::MAX) > obstacles + 1
                {
                    visited.insert((i - 1, j), obstacles + 1);
                    deque.push_back((i - 1, j, obstacles + 1, steps + 1));
                } else if grid[i - 1][j] == 0
                    && *visited.get(&(i - 1, j)).unwrap_or(&i32::MAX) > obstacles
                {
                    visited.insert((i - 1, j), obstacles);
                    deque.push_back((i - 1, j, obstacles, steps + 1));
                }
            }
            if i < m - 1 {
                if grid[i + 1][j] == 1
                    && obstacles + 1 <= k
                    && *visited.get(&(i + 1, j)).unwrap_or(&i32::MAX) > obstacles + 1
                {
                    visited.insert((i + 1, j), obstacles + 1);
                    deque.push_back((i + 1, j, obstacles + 1, steps + 1));
                } else if grid[i + 1][j] == 0
                    && *visited.get(&(i + 1, j)).unwrap_or(&i32::MAX) > obstacles
                {
                    visited.insert((i + 1, j), obstacles);
                    deque.push_back((i + 1, j, obstacles, steps + 1));
                }
            }
            if j > 0 {
                if grid[i][j - 1] == 1
                    && obstacles + 1 <= k
                    && *visited.get(&(i, j - 1)).unwrap_or(&i32::MAX) > obstacles + 1
                {
                    visited.insert((i, j - 1), obstacles + 1);
                    deque.push_back((i, j - 1, obstacles + 1, steps + 1));
                } else if grid[i][j - 1] == 0
                    && *visited.get(&(i, j - 1)).unwrap_or(&i32::MAX) > obstacles
                {
                    visited.insert((i, j - 1), obstacles);
                    deque.push_back((i, j - 1, obstacles, steps + 1));
                }
            }
            if j < n - 1 {
                if grid[i][j + 1] == 1
                    && obstacles + 1 <= k
                    && *visited.get(&(i, j + 1)).unwrap_or(&i32::MAX) > obstacles + 1
                {
                    visited.insert((i, j + 1), obstacles + 1);
                    deque.push_back((i, j + 1, obstacles + 1, steps + 1));
                } else if grid[i][j + 1] == 0
                    && *visited.get(&(i, j + 1)).unwrap_or(&i32::MAX) > obstacles
                {
                    visited.insert((i, j + 1), obstacles);
                    deque.push_back((i, j + 1, obstacles, steps + 1));
                }
            }
        }

        -1
    }
}
