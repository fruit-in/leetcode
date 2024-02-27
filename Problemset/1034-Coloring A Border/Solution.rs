use std::collections::HashSet;

impl Solution {
    pub fn color_border(mut grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let row = row as usize;
        let col = col as usize;
        let mut stack = vec![(row, col)];
        let mut visited = HashSet::from([(row, col)]);
        let mut borders = vec![];

        while let Some((r, c)) = stack.pop() {
            let mut count = 0;

            if r > 0 && grid[r - 1][c] == grid[row][col] {
                count += 1;
                if visited.insert((r - 1, c)) {
                    stack.push((r - 1, c));
                }
            }
            if r < m - 1 && grid[r + 1][c] == grid[row][col] {
                count += 1;
                if visited.insert((r + 1, c)) {
                    stack.push((r + 1, c));
                }
            }
            if c > 0 && grid[r][c - 1] == grid[row][col] {
                count += 1;
                if visited.insert((r, c - 1)) {
                    stack.push((r, c - 1));
                }
            }
            if c < n - 1 && grid[r][c + 1] == grid[row][col] {
                count += 1;
                if visited.insert((r, c + 1)) {
                    stack.push((r, c + 1));
                }
            }

            if count < 4 {
                borders.push((r, c));
            }
        }

        for (r, c) in borders {
            grid[r][c] = color;
        }

        grid
    }
}
