# 2290. Minimum Obstacle Removal to Reach Corner
You are given a **0-indexed** 2D integer array `grid` of size `m x n`. Each cell has one of two values:

* `0` represents an **empty** cell,
* `1` represents an **obstacle** that may be removed.

You can move up, down, left, or right from and to an empty cell.

Return *the **minimum** number of **obstacles** to **remove** so you can move from the upper left corner* `(0, 0)` *to the lower right corner* `(m - 1, n - 1)`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/04/06/example1drawio-1.png)
<pre>
<strong>Input:</strong> grid = [[0,1,1],[1,1,0],[1,1,0]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can remove the obstacles at (0, 1) and (0, 2) to create a path from (0, 0) to (2, 2).
It can be shown that we need to remove at least 2 obstacles, so we return 2.
Note that there may be other ways to remove 2 obstacles to create a path.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/04/06/example1drawio.png)
<pre>
<strong>Input:</strong> grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> We can move from (0, 0) to (2, 4) without removing any obstacles, so we return 0.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>2 <= m * n <= 10<sup>5</sup></code>
* `grid[i][j]` is either `0` **or** `1`.
* `grid[0][0] == grid[m - 1][n - 1] == 0`

## Solutions (Rust)

### 1. Solution
```Rust
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
```
