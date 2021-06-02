# 1738. 找出第 K 大的异或坐标值
给你一个二维矩阵 `matrix` 和一个整数 `k` ，矩阵大小为 `m x n` 由非负整数组成。

矩阵中坐标 `(a, b)` 的 **值** 可由对所有满足 `0 <= i <= a < m` 且 `0 <= j <= b < n` 的元素 `matrix[i][j]`（**下标从 0 开始计数**）执行异或运算得到。

请你找出 `matrix` 的所有坐标中第 `k` 大的值（**`k` 的值从 1 开始计数**）。

#### 示例 1:
<pre>
<strong>输入:</strong> matrix = [[5,2],[1,6]], k = 1
<strong>输出:</strong> 7
<strong>解释:</strong> 坐标 (0,1) 的值是 5 XOR 2 = 7 ，为最大的值。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matrix = [[5,2],[1,6]], k = 2
<strong>输出:</strong> 5
<strong>解释:</strong> 坐标 (0,0) 的值是 5 = 5 ，为第 2 大的值。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> matrix = [[5,2],[1,6]], k = 3
<strong>输出:</strong> 4
<strong>解释:</strong> 坐标 (1,0) 的值是 5 XOR 1 = 4 ，为第 3 大的值。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> matrix = [[5,2],[1,6]], k = 4
<strong>输出:</strong> 0
<strong>解释:</strong> 坐标 (1,1) 的值是 5 XOR 2 XOR 1 XOR 6 = 0 ，为第 4 大的值。
</pre>

#### 提示:
* `m == matrix.length`
* `n == matrix[i].length`
* `1 <= m, n <= 1000`
* <code>0 <= matrix[i][j] <= 10<sup>6</sup></code>
* `1 <= k <= m * n`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} matrix
# @param {Integer} k
# @return {Integer}
def kth_largest_value(matrix, k)
  vals = []

  (0...matrix.size).each do |i|
    (0...matrix[0].size).each do |j|
      matrix[i][j] ^= matrix[i - 1][j] if i > 0
      matrix[i][j] ^= matrix[i][j - 1] if j > 0
      matrix[i][j] ^= matrix[i - 1][j - 1] if i > 0 && j > 0
      vals.push(matrix[i][j])
    end
  end

  vals.sort[-k]
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn kth_largest_value(mut matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut vals = vec![];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 {
                    matrix[i][j] ^= matrix[i - 1][j];
                }
                if j > 0 {
                    matrix[i][j] ^= matrix[i][j - 1];
                }
                if i > 0 && j > 0 {
                    matrix[i][j] ^= matrix[i - 1][j - 1];
                }
                vals.push(matrix[i][j]);
            }
        }

        vals.sort_unstable_by(|a, b| b.cmp(a));

        vals[k as usize - 1]
    }
}
```
