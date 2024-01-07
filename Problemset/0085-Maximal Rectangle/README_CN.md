# 85. 最大矩形
给定一个仅包含 `0` 和 `1` 、大小为 `rows x cols` 的二维二进制矩阵，找出只包含 `1` 的最大矩形，并返回其面积。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/14/maximal.jpg)
<pre>
<strong>输入:</strong> matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
<strong>输出:</strong> 6
<strong>解释:</strong> 最大矩形如上图所示。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matrix = [["0"]]
<strong>输出:</strong> 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> matrix = [["1"]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `rows == matrix.length`
* `cols == matrix[i].length`
* `1 <= row, cols <= 200`
* `matrix[i][j]` 为 `'0'` 或 `'1'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut prefix_sum = vec![vec![0; cols]; rows];
        let mut ret = 0;

        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == '0' {
                    continue;
                }

                prefix_sum[r][c] = 1;

                if c > 0 {
                    prefix_sum[r][c] += prefix_sum[r][c - 1];
                }

                let mut min_w = i32::MAX;

                for h in 1..=r + 1 {
                    if prefix_sum[r + 1 - h][c] == 0 {
                        break;
                    }

                    min_w = min_w.min(prefix_sum[r + 1 - h][c]);
                    ret = ret.max(min_w * h as i32);
                }
            }
        }

        ret
    }
}
```
