# 1034. Coloring A Border
You are given an `m x n` integer matrix `grid`, and three integers `row`, `col`, and `color`. Each value in the grid represents the color of the grid square at that location.

Two squares are called **adjacent** if they are next to each other in any of the 4 directions.

Two squares belong to the same **connected component** if they have the same color and they are adjacent.

The **border of a connected component** is all the squares in the connected component that are either adjacent to (at least) a square not in the component, or on the boundary of the grid (the first or last row or column).

You should color the **border** of the **connected component** that contains the square `grid[row][col]` with `color`.

Return *the final grid*.

#### Example 1:
<pre>
<strong>Input:</strong> grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
<strong>Output:</strong> [[3,3],[3,2]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
<strong>Output:</strong> [[1,3,3],[2,3,3]]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
<strong>Output:</strong> [[2,2,2],[2,1,2],[2,2,2]]
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* `1 <= grid[i][j], color <= 1000`
* `0 <= row < m`
* `0 <= col < n`

## Solutions (Rust)

### 1. Solution
```Rust
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
```
