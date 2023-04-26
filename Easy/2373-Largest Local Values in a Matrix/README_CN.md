# 2373. 矩阵中的局部最大值
给你一个大小为 `n x n` 的整数矩阵 `grid` 。

生成一个大小为 `(n - 2) x (n - 2)` 的整数矩阵  `maxLocal` ，并满足：

* `maxLocal[i][j]` 等于 `grid` 中以 `i + 1` 行和 `j + 1` 列为中心的 `3 x 3` 矩阵中的 **最大值** 。

换句话说，我们希望找出 `grid` 中每个 `3 x 3` 矩阵中的最大值。

返回生成的矩阵。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/06/21/ex1.png)
<pre>
<strong>输入:</strong> grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
<strong>输出:</strong> [[9,9],[8,6]]
<strong>解释:</strong> 原矩阵和生成的矩阵如上图所示。
注意，生成的矩阵中，每个值都对应 grid 中一个相接的 3 x 3 矩阵的最大值。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/07/02/ex2new2.png)
<pre>
<strong>输入:</strong> grid = [[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]]
<strong>输出:</strong> [[2,2,2],[2,2,2],[2,2,2]]
<strong>解释:</strong> 注意，2 包含在 grid 中每个 3 x 3 的矩阵中。
</pre>

#### 提示:
* `n == grid.length == grid[i].length`
* `3 <= n <= 100`
* `1 <= grid[i][j] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut max_local = vec![vec![0; n - 2]; n - 2];

        for i in 0..n - 2 {
            for j in 0..n - 2 {
                for a in 0..3 {
                    for b in 0..3 {
                        max_local[i][j] = max_local[i][j].max(grid[i + a][j + b]);
                    }
                }
            }
        }

        max_local
    }
}
```
