# 1605. 给定行和列的和求可行矩阵
给你两个非负整数数组 `rowSum` 和 `colSum` ，其中 `rowSum[i]` 是二维矩阵中第 `i` 行元素的和， `colSum[j]` 是第 `j` 列元素的和。换言之你不知道矩阵里的每个元素，但是你知道每一行和每一列的和。

请找到大小为 `rowSum.length x colSum.length` 的任意 **非负整数** 矩阵，且该矩阵满足 `rowSum` 和 `colSum` 的要求。

请你返回任意一个满足题目要求的二维矩阵，题目保证存在 **至少一个** 可行矩阵。

#### 示例 1:
<pre>
<strong>输入:</strong> rowSum = [3,8], colSum = [4,7]
<strong>输出:</strong> [[3,0],
         [1,7]]
<strong>解释:</strong>
第 0 行：3 + 0 = 3 == rowSum[0]
第 1 行：1 + 7 = 8 == rowSum[1]
第 0 列：3 + 1 = 4 == colSum[0]
第 1 列：0 + 7 = 7 == colSum[1]
行和列的和都满足题目要求，且所有矩阵元素都是非负的。
另一个可行的矩阵为：[[1,2],
                  [3,5]]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rowSum = [5,7,10], colSum = [8,6,8]
<strong>输出:</strong> [[0,5,0],
      [6,1,0],
      [2,0,8]]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> rowSum = [14,9], colSum = [6,9,8]
<strong>输出:</strong> [[0,9,5],
      [6,0,3]]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> rowSum = [1,0], colSum = [1]
<strong>输出:</strong> [[1],
      [0]]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> rowSum = [0], colSum = [0]
<strong>输出:</strong> [[0]]
</pre>

#### 提示:
* `1 <= rowSum.length, colSum.length <= 500`
* <code>0 <= rowSum[i], colSum[i] <= 10<sup>8</sup></code>
* `sum(rowSum) == sum(colSum)`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut row = 0;
        let mut col = 0;
        let mut ret = vec![vec![0; col_sum.len()]; row_sum.len()];

        while row < row_sum.len() && col < col_sum.len() {
            if row_sum[row] < col_sum[col] {
                ret[row][col] = row_sum[row];
                col_sum[col] -= row_sum[row];
                row += 1;
            } else if row_sum[row] > col_sum[col] {
                ret[row][col] = col_sum[col];
                row_sum[row] -= col_sum[col];
                col += 1;
            } else {
                ret[row][col] = row_sum[row];
                row += 1;
                col += 1;
            }
        }

        ret
    }
}
```
