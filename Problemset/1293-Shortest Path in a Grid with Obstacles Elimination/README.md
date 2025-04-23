# 1293. Shortest Path in a Grid with Obstacles Elimination
You are given an `m x n` integer matrix `grid` where each cell is either `0` (empty) or `1` (obstacle). You can move up, down, left, or right from and to an empty cell in **one step**.

Return *the minimum number of steps to walk from the upper left corner* `(0, 0)` *to the lower right corner* `(m - 1, n - 1)` *given that you can eliminate **at most*** `k` *obstacles*. If it is not possible to find such walk return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/09/30/short1-grid.jpg)
<pre>
<strong>Input:</strong> grid = [[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], k = 1
<strong>Output:</strong> 6
<strong>Explanation:</strong>
The shortest path without eliminating any obstacle is 10.
The shortest path with one obstacle elimination at position (3,2) is 6. Such path is (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2) -> (3,2) -> (4,2).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/09/30/short2-grid.jpg)
<pre>
<strong>Input:</strong> grid = [[0,1,1],[1,1,1],[1,0,0]], k = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong> We need to eliminate at least two obstacles to find such a walk.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 40`
* `1 <= k <= m * n`
* `grid[i][j]` is either `0` **or** `1`.
* `grid[0][0] == grid[m - 1][n - 1] == 0`

## Solutions (Rust)

### 1. Solution
```Rust
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
```
