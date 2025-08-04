impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = vec![];
        let mut ret = 0;

        for r in 0..m {
            if grid[r][0] == 0 {
                grid[r][0] = 1;
                stack.push((r, 0));
            }
            if grid[r][n - 1] == 0 {
                grid[r][n - 1] = 1;
                stack.push((r, n - 1));
            }
        }
        for c in 1..n - 1 {
            if grid[0][c] == 0 {
                grid[0][c] = 1;
                stack.push((0, c));
            }
            if grid[m - 1][c] == 0 {
                grid[m - 1][c] = 1;
                stack.push((m - 1, c));
            }
        }

        while let Some((r, c)) = stack.pop() {
            if r > 0 && grid[r - 1][c] == 0 {
                grid[r - 1][c] = 1;
                stack.push((r - 1, c));
            }
            if r < m - 1 && grid[r + 1][c] == 0 {
                grid[r + 1][c] = 1;
                stack.push((r + 1, c));
            }
            if c > 0 && grid[r][c - 1] == 0 {
                grid[r][c - 1] = 1;
                stack.push((r, c - 1));
            }
            if c < n - 1 && grid[r][c + 1] == 0 {
                grid[r][c + 1] = 1;
                stack.push((r, c + 1));
            }
        }

        for r in 1..m - 1 {
            for c in 1..n - 1 {
                if grid[r][c] == 1 {
                    continue;
                }

                grid[r][c] = 1;
                stack.push((r, c));
                while let Some((r, c)) = stack.pop() {
                    if r > 0 && grid[r - 1][c] == 0 {
                        grid[r - 1][c] = 1;
                        stack.push((r - 1, c));
                    }
                    if r < m - 1 && grid[r + 1][c] == 0 {
                        grid[r + 1][c] = 1;
                        stack.push((r + 1, c));
                    }
                    if c > 0 && grid[r][c - 1] == 0 {
                        grid[r][c - 1] = 1;
                        stack.push((r, c - 1));
                    }
                    if c < n - 1 && grid[r][c + 1] == 0 {
                        grid[r][c + 1] = 1;
                        stack.push((r, c + 1));
                    }
                }

                ret += 1;
            }
        }

        ret
    }
}
