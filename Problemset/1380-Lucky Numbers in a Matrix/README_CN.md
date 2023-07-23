# 1380. 矩阵中的幸运数
给你一个 ```m * n``` 的矩阵，矩阵中的数字 **各不相同** 。请你按 **任意** 顺序返回矩阵中的所有幸运数。

幸运数是指矩阵中满足同时下列两个条件的元素：
* 在同一行的所有元素中最小
* 在同一列的所有元素中最大

#### 示例 1:
<pre>
<strong>输入:</strong> matrix = [[3,7,8],[9,11,13],[15,16,17]]
<strong>输出:</strong> [15]
<strong>解释:</strong> 15 是唯一的幸运数，因为它是其所在行中的最小值，也是所在列中的最大值。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
<strong>输出:</strong> [12]
<strong>解释:</strong> 12 是唯一的幸运数，因为它是其所在行中的最小值，也是所在列中的最大值。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> matrix = [[7,8],[1,2]]
<strong>输出:</strong> [7]
</pre>

#### 提示:
* ```m == mat.length```
* ```n == mat[i].length```
* ```1 <= n, m <= 50```
* ```1 <= matrix[i][j] <= 10^5```
* 矩阵中的所有元素都是不同的

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row_min = vec![100001; m];
        let mut col_max = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                row_min[i] = row_min[i].min(matrix[i][j]);
                col_max[j] = col_max[j].max(matrix[i][j]);
            }
        }

        row_min.drain(..).filter(|x| col_max.contains(x)).collect()
    }
}
```
