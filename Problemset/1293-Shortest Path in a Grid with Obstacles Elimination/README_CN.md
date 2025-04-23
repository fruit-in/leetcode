# 1293. 网格中的最短路径
给你一个 `m * n` 的网格，其中每个单元格不是 `0`（空）就是 `1`（障碍物）。每一步，您都可以在空白单元格中上、下、左、右移动。

如果您 **最多** 可以消除 `k` 个障碍物，请找出从左上角 `(0, 0)` 到右下角 `(m-1, n-1)` 的最短路径，并返回通过该路径所需的步数。如果找不到这样的路径，则返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/09/30/short1-grid.jpg)
<pre>
<strong>输入:</strong> grid = [[0,0,0],[1,1,0],[0,0,0],[0,1,1],[0,0,0]], k = 1
<strong>输出:</strong> 6
<strong>解释:</strong>
不消除任何障碍的最短路径是 10。
消除位置 (3,2) 处的障碍后，最短路径是 6 。该路径是 (0,0) -> (0,1) -> (0,2) -> (1,2) -> (2,2) -> (3,2) -> (4,2).
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/09/30/short2-grid.jpg)
<pre>
<strong>输入:</strong> grid = [[0,1,1],[1,1,1],[1,0,0]], k = 1
<strong>输出:</strong> -1
<strong>解释:</strong> 我们至少需要消除两个障碍才能找到这样的路径。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 40`
* `1 <= k <= m * n`
* `grid[i][j]` 是 `0` 或 `1`
* `grid[0][0] == grid[m - 1][n - 1] == 0`

## 题解 (Rust)

### 1. 题解
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
