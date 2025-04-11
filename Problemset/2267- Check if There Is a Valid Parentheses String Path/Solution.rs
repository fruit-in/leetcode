use std::collections::HashSet;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = vec![(0, 0, 1)];
        let mut visited = HashSet::from([(0, 0, 1)]);

        if grid[0][0] == ')' || grid[m - 1][n - 1] == '(' || (m + n) % 2 == 0 {
            return false;
        }

        while let Some((i, j, diff)) = stack.pop() {
            if i == m - 1 && j == n - 1 && diff == 0 {
                return true;
            }

            if i + 1 < m {
                let new_diff = diff + 81 - grid[i + 1][j] as i32 * 2;
                if new_diff >= 0 && !visited.contains(&(i + 1, j, new_diff)) {
                    stack.push((i + 1, j, new_diff));
                    visited.insert((i + 1, j, new_diff));
                }
            }

            if j + 1 < n {
                let new_diff = diff + 81 - grid[i][j + 1] as i32 * 2;
                if new_diff >= 0 && !visited.contains(&(i, j + 1, new_diff)) {
                    stack.push((i, j + 1, new_diff));
                    visited.insert((i, j + 1, new_diff));
                }
            }
        }

        false
    }
}
