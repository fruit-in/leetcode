use std::collections::HashSet;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut seen = HashSet::new();
        let mut cells = vec![(0, 0)];

        while let Some((i, j)) = cells.pop() {
            if i == m - 1 && j == n - 1 {
                return true;
            }

            seen.insert((i, j));

            match grid[i][j] {
                1 => {
                    if j > 0 && !seen.contains(&(i, j - 1)) && [1, 4, 6].contains(&grid[i][j - 1]) {
                        cells.push((i, j - 1));
                    }
                    if j + 1 < n
                        && !seen.contains(&(i, j + 1))
                        && [1, 3, 5].contains(&grid[i][j + 1])
                    {
                        cells.push((i, j + 1));
                    }
                }
                2 => {
                    if i > 0 && !seen.contains(&(i - 1, j)) && [2, 3, 4].contains(&grid[i - 1][j]) {
                        cells.push((i - 1, j));
                    }
                    if i + 1 < m
                        && !seen.contains(&(i + 1, j))
                        && [2, 5, 6].contains(&grid[i + 1][j])
                    {
                        cells.push((i + 1, j));
                    }
                }
                3 => {
                    if j > 0 && !seen.contains(&(i, j - 1)) && [1, 4, 6].contains(&grid[i][j - 1]) {
                        cells.push((i, j - 1));
                    }
                    if i + 1 < m
                        && !seen.contains(&(i + 1, j))
                        && [2, 5, 6].contains(&grid[i + 1][j])
                    {
                        cells.push((i + 1, j));
                    }
                }
                4 => {
                    if j + 1 < n
                        && !seen.contains(&(i, j + 1))
                        && [1, 3, 5].contains(&grid[i][j + 1])
                    {
                        cells.push((i, j + 1));
                    }
                    if i + 1 < m
                        && !seen.contains(&(i + 1, j))
                        && [2, 5, 6].contains(&grid[i + 1][j])
                    {
                        cells.push((i + 1, j));
                    }
                }
                5 => {
                    if i > 0 && !seen.contains(&(i - 1, j)) && [2, 3, 4].contains(&grid[i - 1][j]) {
                        cells.push((i - 1, j));
                    }
                    if j > 0 && !seen.contains(&(i, j - 1)) && [1, 4, 6].contains(&grid[i][j - 1]) {
                        cells.push((i, j - 1));
                    }
                }
                _ => {
                    if i > 0 && !seen.contains(&(i - 1, j)) && [2, 3, 4].contains(&grid[i - 1][j]) {
                        cells.push((i - 1, j));
                    }
                    if j + 1 < n
                        && !seen.contains(&(i, j + 1))
                        && [1, 3, 5].contains(&grid[i][j + 1])
                    {
                        cells.push((i, j + 1));
                    }
                }
            }
        }

        false
    }
}
