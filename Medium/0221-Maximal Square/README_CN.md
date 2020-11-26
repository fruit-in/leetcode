# 221. 最大正方形
在一个由 `'0'` 和 `'1'` 组成的二维矩阵内，找到只包含 `'1'` 的最大正方形，并返回其面积。

#### 示例:
<pre>
<strong>输入:</strong>
matrix = [["1","0","1","0","0"],
          ["1","0","<b>1</b>","<b>1</b>","1"],
          ["1","1","<b>1</b>","<b>1</b>","1"],
          ["1","0","0","1","0"]]
<strong>输出:</strong> 4
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut matrix = matrix
            .iter()
            .map(|row| row.iter().map(|&c| c as i32 - 48).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] += matrix[i - 1][j - 1]
                        .min(matrix[i][j - 1])
                        .min(matrix[i - 1][j]);
                }
                ret = ret.max(matrix[i][j]);
            }
        }

        ret * ret
    }
}
```
