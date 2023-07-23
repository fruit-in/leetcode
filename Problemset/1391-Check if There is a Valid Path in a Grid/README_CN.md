# 1391. 检查网格中是否存在有效路径
给你一个 *m* x *n* 的网格 `grid`。网格里的每个单元都代表一条街道。`grid[i][j]` 的街道可以是：
* **1** 表示连接左单元格和右单元格的街道。
* **2** 表示连接上单元格和下单元格的街道。
* **3** 表示连接左单元格和下单元格的街道。
* **4** 表示连接右单元格和下单元格的街道。
* **5** 表示连接左单元格和上单元格的街道。
* **6** 表示连接右单元格和上单元格的街道。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/03/21/main.png)

你最开始从左上角的单元格 `(0,0)` 开始出发，网格中的「有效路径」是指从左上方的单元格 `(0,0)` 开始、一直到右下方的 `(m-1,n-1)` 结束的路径。**该路径必须只沿着街道走**。

**注意:** 你 **不能** 变更街道。

如果网格中存在有效的路径，则返回 `true`，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/03/05/e1.png)
<pre>
<strong>输入:</strong> grid = [[2,4,3],[6,5,2]]
<strong>输出:</strong> true
<strong>解释:</strong> 如图所示，你可以从 (0, 0) 开始，访问网格中的所有单元格并到达 (m - 1, n - 1) 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/03/05/e2.png)
<pre>
<strong>输入:</strong> grid = [[1,2,1],[1,2,1]]
<strong>输出:</strong> false
<strong>解释:</strong> 如图所示，单元格 (0, 0) 上的街道没有与任何其他单元格上的街道相连，你只会停在 (0, 0) 处。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,1,2]]
<strong>输出:</strong> false
<strong>Explanation:</strong> 你会停在 (0, 1)，而且无法到达 (0, 2) 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> grid = [[1,1,1,1,1,1,3]]
<strong>输出:</strong> true
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> grid = [[2],[2],[2],[2],[2],[2],[6]]
<strong>输出:</strong> true
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 300`
* `1 <= grid[i][j] <= 6`

## 题解 (Rust)

### 1. 题解
```Rust
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
```
