# 2556. 二进制矩阵中翻转最多一次使路径不连通
给你一个下标从 **0** 开始的 `m x n` **二进制** 矩阵 `grid` 。你可以从一个格子 `(row, col)` 移动到格子 `(row + 1, col)` 或者 `(row, col + 1)` ，前提是前往的格子值为 `1` 。如果从 `(0, 0)` 到 `(m - 1, n - 1)` 没有任何路径，我们称该矩阵是 **不连通** 的。

你可以翻转 **最多一个** 格子的值（也可以不翻转）。你 **不能翻转** 格子 `(0, 0)` 和 `(m - 1, n - 1)` 。

如果可以使矩阵不连通，请你返回 `true` ，否则返回 `false` 。

**注意** ，翻转一个格子的值，可以使它的值从 `0` 变 `1` ，或从 `1` 变 `0` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/12/07/yetgrid2drawio.png)
<pre>
<strong>输入:</strong> grid = [[1,1,1],[1,0,0],[1,1,1]]
<strong>输出:</strong> true
<strong>解释:</strong> 按照上图所示我们翻转蓝色格子里的值，翻转后从 (0, 0) 到 (2, 2) 没有路径。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/12/07/yetgrid3drawio.png)
<pre>
<strong>输入:</strong> grid = [[1,1,1],[1,0,1],[1,1,1]]
<strong>输出:</strong> false
<strong>解释:</strong> 无法翻转至多一个格子，使 (0, 0) 到 (2, 2) 没有路径。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 1000`
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `grid[0][0] == grid[m - 1][n - 1] == 1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_possible_to_cut_path(mut grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let mut connect = vec![vec![false; n]; m];
        connect[0][0] = true;

        for r in 0..m {
            for c in 0..n {
                connect[r][c] |= grid[r][c] == 1
                    && ((r > 0 && connect[r - 1][c]) || (c > 0 && connect[r][c - 1]));
            }
        }

        if !connect[m - 1][n - 1] {
            return true;
        }

        let mut r = m - 1;
        let mut c = n - 1;

        while r > 0 || c > 0 {
            if r > 0 && connect[r - 1][c] {
                r -= 1;
            } else {
                c -= 1;
            }
            grid[r][c] = 0;
        }

        connect = vec![vec![false; n]; m];
        connect[0][0] = true;

        for r in 0..m {
            for c in 0..n {
                connect[r][c] |= grid[r][c] == 1
                    && ((r > 0 && connect[r - 1][c]) || (c > 0 && connect[r][c - 1]));
            }
        }

        !connect[m - 1][n - 1]
    }
}
```
