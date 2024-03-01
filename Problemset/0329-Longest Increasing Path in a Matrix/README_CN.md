# 329. 矩阵中的最长递增路径
给定一个 `m x n` 整数矩阵 `matrix` ，找出其中 **最长递增路径** 的长度。

对于每个单元格，你可以往上，下，左，右四个方向移动。 你 **不能** 在 **对角线** 方向上移动或移动到 **边界外**（即不允许环绕）。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/01/05/grid1.jpg)
<pre>
<strong>输入:</strong> matrix = [[9,9,4],[6,6,8],[2,1,1]]
<strong>输出:</strong> 4
<strong>解释:</strong> 最长递增路径为 [1, 2, 6, 9]。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/01/27/tmp-grid.jpg)
<pre>
<strong>输入:</strong> matrix = [[3,4,5],[3,2,6],[2,2,1]]
<strong>输出:</strong> 4
<strong>解释:</strong> 最长递增路径是 [3, 4, 5, 6]。注意不允许在对角线方向上移动。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> matrix = [[1]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `m == matrix.length`
* `n == matrix[i].length`
* `1 <= m, n <= 200`
* <code>0 <= matrix[i][j] <= 2<sup>31</sup> - 1</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut cells = vec![];
        let mut dp = vec![vec![1; n]; m];
        let mut ret = 1;

        for r in 0..m {
            for c in 0..n {
                cells.push((r, c));
            }
        }

        cells.sort_unstable_by_key(|&(r, c)| matrix[r][c]);

        for (r, c) in cells {
            let mut max_path = 0;

            if r > 0 && matrix[r - 1][c] < matrix[r][c] {
                max_path = max_path.max(dp[r - 1][c]);
            }
            if r < m - 1 && matrix[r + 1][c] < matrix[r][c] {
                max_path = max_path.max(dp[r + 1][c]);
            }
            if c > 0 && matrix[r][c - 1] < matrix[r][c] {
                max_path = max_path.max(dp[r][c - 1]);
            }
            if c < n - 1 && matrix[r][c + 1] < matrix[r][c] {
                max_path = max_path.max(dp[r][c + 1]);
            }

            dp[r][c] = max_path + 1;
            ret = ret.max(dp[r][c]);
        }

        ret
    }
}
```
