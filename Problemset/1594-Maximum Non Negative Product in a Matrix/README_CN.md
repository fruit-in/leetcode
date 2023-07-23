# 1594. 矩阵的最大非负积
给你一个大小为 `rows x cols` 的矩阵 `grid` 。最初，你位于左上角 `(0, 0)` ，每一步，你可以在矩阵中 **向右** 或 **向下** 移动。

在从左上角 `(0, 0)` 开始到右下角 `(rows - 1, cols - 1)` 结束的所有路径中，找出具有 **最大非负积** 的路径。路径的积是沿路径访问的单元格中所有整数的乘积。

返回 **最大非负积** 对 <code><b>10<sup>9</sup> + 7</b></code> **取余** 的结果。如果最大积为负数，则返回 -1 。

**注意**，取余是在得到最大积之后执行的。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[-1,-2,-3],
             [-2,-3,-3],
             [-3,-3,-2]]
<strong>输出:</strong> -1
<strong>解释:</strong> 从 (0, 0) 到 (2, 2) 的路径中无法得到非负积，所以返回 -1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[1,-2,1],
             [1,-2,1],
             [3,-4,1]]
<strong>输出:</strong> 8
<strong>解释:</strong> 最大非负积对应的路径已经用粗体标出 (1 * 1 * -2 * -4 * 1 = 8)
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1, 3],
             [0,-4]]
<strong>输出:</strong> 0
<strong>解释:</strong> 最大非负积对应的路径已经用粗体标出 (1 * 0 * -4 = 0)
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> grid = [[ 1, 4,4,0],
             [-2, 0,0,1],
             [ 1,-1,1,1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 最大非负积对应的路径已经用粗体标出 (1 * -2 * 1 * -1 * 1 * 1 = 2)
</pre>

#### 提示:
* `1 <= rows, cols <= 15`
* `-4 <= grid[i][j] <= 4`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut products = vec![vec![[1, -1]; n]; m];
        products[0][0] = [grid[0][0].min(1) as i64, grid[0][0].max(-1) as i64];

        for i in 0..m {
            for j in 0..n {
                if i + 1 < m {
                    if grid[i + 1][j] > 0 {
                        products[i + 1][j][0] =
                            products[i + 1][j][0].min(products[i][j][0] * grid[i + 1][j] as i64);
                        products[i + 1][j][1] =
                            products[i + 1][j][1].max(products[i][j][1] * grid[i + 1][j] as i64);
                    } else if grid[i + 1][j] < 0 {
                        products[i + 1][j][0] =
                            products[i + 1][j][0].min(products[i][j][1] * grid[i + 1][j] as i64);
                        products[i + 1][j][1] =
                            products[i + 1][j][1].max(products[i][j][0] * grid[i + 1][j] as i64);
                    } else {
                        products[i + 1][j] = [0, 0];
                    }
                }
                if j + 1 < n {
                    if grid[i][j + 1] > 0 {
                        products[i][j + 1][0] =
                            products[i][j + 1][0].min(products[i][j][0] * grid[i][j + 1] as i64);
                        products[i][j + 1][1] =
                            products[i][j + 1][1].max(products[i][j][1] * grid[i][j + 1] as i64);
                    } else if grid[i][j + 1] < 0 {
                        products[i][j + 1][0] =
                            products[i][j + 1][0].min(products[i][j][1] * grid[i][j + 1] as i64);
                        products[i][j + 1][1] =
                            products[i][j + 1][1].max(products[i][j][0] * grid[i][j + 1] as i64);
                    } else {
                        products[i][j + 1] = [0, 0];
                    }
                }
            }
        }

        (products[m - 1][n - 1][1] % 1_000_000_007) as i32
    }
}
```
