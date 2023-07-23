# 1277. 统计全为 1 的正方形子矩阵
给你一个 `m * n` 的矩阵，矩阵中的元素不是 `0` 就是 `1`，请你统计并返回其中完全由 `1` 组成的 **正方形** 子矩阵的个数。

#### 示例 1:
<pre>
<strong>输入:</strong> matrix =
[
  [0,1,1,1],
  [1,1,1,1],
  [0,1,1,1]
]
<strong>输出:</strong> 15
<strong>解释:</strong>
边长为 1 的正方形有 <b>10</b> 个。
边长为 2 的正方形有 <b>4</b> 个。
边长为 3 的正方形有 <b>1</b> 个。
正方形的总数 = 10 + 4 + 1 = <b>15</b>.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matrix =
[
  [1,0,1],
  [1,1,0],
  [1,1,0]
]
<strong>输出:</strong> 7
<strong>解释:</strong>
边长为 1 的正方形有 <b>6</b> 个。
边长为 2 的正方形有 <b>1</b> 个。
正方形的总数 = 6 + 1 = <b>7</b>.
</pre>

#### 提示:
* `1 <= arr.length <= 300`
* `1 <= arr[0].length <= 300`
* `0 <= arr[i][j] <= 1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] +=
                        matrix[i - 1][j - 1].min(matrix[i][j - 1].min(matrix[i - 1][j]));
                }

                ret += matrix[i][j];
            }
        }

        ret
    }
}
```
