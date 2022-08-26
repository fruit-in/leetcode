# 1975. 最大方阵和
给你一个 `n x n` 的整数方阵 `matrix` 。你可以执行以下操作 **任意次** ：
* 选择 `matrix` 中 **相邻** 两个元素，并将它们都 **乘以** `-1` 。

如果两个元素有 **公共边** ，那么它们就是 **相邻** 的。

你的目的是 **最大化** 方阵元素的和。请你在执行以上操作之后，返回方阵的 **最大** 和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/07/16/pc79-q2ex1.png)
<pre>
<strong>输入:</strong> matrix = [[1,-1],[-1,1]]
<strong>输出:</strong> 4
<strong>解释:</strong> 我们可以执行以下操作使和等于 4 ：
- 将第一行的 2 个元素乘以 -1 。
- 将第一列的 2 个元素乘以 -1 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/07/16/pc79-q2ex2.png)
<pre>
<strong>输入:</strong> matrix = [[1,2,3],[-1,-2,-3],[1,2,3]]
<strong>输出:</strong> 16
<strong>解释:</strong> 我们可以执行以下操作使和等于 16 ：
- 将第二行的最后 2 个元素乘以 -1 。
</pre>

#### 提示:
* `n == matrix.length == matrix[i].length`
* `2 <= n <= 250`
* <code>-10<sup>5</sup> <= matrix[i][j] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut count_neg_even = true;
        let mut min_abs = i32::MAX;
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix.len() {
                if matrix[i][j] < 0 {
                    count_neg_even = !count_neg_even;
                }
                min_abs = min_abs.min(matrix[i][j].abs());
                ret += matrix[i][j].abs() as i64;
            }
        }

        if !count_neg_even {
            ret -= 2 * min_abs as i64;
        }

        ret
    }
}
```
