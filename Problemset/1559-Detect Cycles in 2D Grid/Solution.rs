use std::collections::HashSet;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited0 = HashSet::new();

        for r in 0..m {
            for c in 0..n {
                if visited0.contains(&(r, c)) {
                    continue;
                }

                let value = grid[r][c];
                let mut stack = vec![('\0', r, c)];
                let mut visited1 = HashSet::from([(r, c)]);

                while let Some((from, r, c)) = stack.pop() {
                    if r > 0 && grid[r - 1][c] == value && from != 'U' {
                        if visited1.contains(&(r - 1, c)) {
                            return true;
                        }
                        stack.push(('D', r - 1, c));
                        visited1.insert((r - 1, c));
                    }
                    if r < m - 1 && grid[r + 1][c] == value && from != 'D' {
                        if visited1.contains(&(r + 1, c)) {
                            return true;
                        }
                        stack.push(('U', r + 1, c));
                        visited1.insert((r + 1, c));
                    }
                    if c > 0 && grid[r][c - 1] == value && from != 'L' {
                        if visited1.contains(&(r, c - 1)) {
                            return true;
                        }
                        stack.push(('R', r, c - 1));
                        visited1.insert((r, c - 1));
                    }
                    if c < n - 1 && grid[r][c + 1] == value && from != 'R' {
                        if visited1.contains(&(r, c + 1)) {
                            return true;
                        }
                        stack.push(('L', r, c + 1));
                        visited1.insert((r, c + 1));
                    }
                }

                visited0.extend(visited1);
            }
        }

        false
    }
}
