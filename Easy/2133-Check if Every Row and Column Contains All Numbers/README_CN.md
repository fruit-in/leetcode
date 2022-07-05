# 2133. 检查是否每一行每一列都包含全部整数
对一个大小为 `n x n` 的矩阵而言，如果其每一行和每一列都包含从 `1` 到 `n` 的 **全部** 整数（含 `1` 和 `n`），则认为该矩阵是一个 **有效** 矩阵。

给你一个大小为 `n x n` 的整数矩阵 `matrix` ，请你判断矩阵是否为一个有效矩阵：如果是，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/21/example1drawio.png)
<pre>
<strong>输入:</strong> matrix = [[1,2,3],[3,1,2],[2,3,1]]
<strong>输出:</strong> true
<strong>解释:</strong> 在此例中，n = 3 ，每一行和每一列都包含数字 1、2、3 。
因此，返回 true 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/21/example2drawio.png)
<pre>
<strong>输入:</strong> matrix = [[1,1,1],[1,2,3],[1,2,3]]
<strong>输出:</strong> false
<strong>解释:</strong> 在此例中，n = 3 ，但第一行和第一列不包含数字 2 和 3 。
因此，返回 false 。
</pre>

#### 提示:
* `n == matrix.length == matrix[i].length`
* `1 <= n <= 100`
* `1 <= matrix[i][j] <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();

        for i in 0..n {
            let mut valid_r = vec![false; n];
            let mut valid_c = vec![false; n];

            for j in 0..n {
                valid_r[matrix[i][j] as usize - 1] = true;
                valid_c[matrix[j][i] as usize - 1] = true;
            }

            if !(0..n).all(|k| valid_r[k] && valid_c[k]) {
                return false;
            }
        }

        true
    }
}
```
