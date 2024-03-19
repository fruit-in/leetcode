# 2290. 到达角落需要移除障碍物的最小数目
给你一个下标从 **0** 开始的二维整数数组 `grid` ，数组大小为 `m x n` 。每个单元格都是两个值之一：

* `0` 表示一个 **空** 单元格，
* `1` 表示一个可以移除的 **障碍物** 。

你可以向上、下、左、右移动，从一个空单元格移动到另一个空单元格。

现在你需要从左上角 `(0, 0)` 移动到右下角 `(m - 1, n - 1)` ，返回需要移除的障碍物的 **最小** 数目。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/04/06/example1drawio-1.png)
<pre>
<strong>输入:</strong> grid = [[0,1,1],[1,1,0],[1,1,0]]
<strong>输出:</strong> 2
<strong>解释:</strong> 可以移除位于 (0, 1) 和 (0, 2) 的障碍物来创建从 (0, 0) 到 (2, 2) 的路径。
可以证明我们至少需要移除两个障碍物，所以返回 2 。
注意，可能存在其他方式来移除 2 个障碍物，创建出可行的路径。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/04/06/example1drawio.png)
<pre>
<strong>输入:</strong> grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]]
<strong>输出:</strong> 0
<strong>解释:</strong> 不移除任何障碍物就能从 (0, 0) 到 (2, 4) ，所以返回 0 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>2 <= m * n <= 10<sup>5</sup></code>
* `grid[i][j]` 为 `0` **或** `1`
* `grid[0][0] == grid[m - 1][n - 1] == 0`

## 题解 (Rust)

### 1. 题解
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
