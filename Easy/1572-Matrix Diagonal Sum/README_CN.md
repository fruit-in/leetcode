# 1572. 矩阵对角线元素的和
给你一个正方形矩阵 `mat`，请你返回矩阵对角线元素的和。

请你返回在矩阵主对角线上的元素和副对角线上且不在主对角线上元素的和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/08/14/sample_1911.png)
<pre>
<b>输入:</b> mat = [[<b>1</b>,2,<b>3</b>],
            [4,<b>5</b>,6],
            [<b>7</b>,8,<b>9</b>]]
<b>输出:</b> 25
<b>解释:</b> 对角线的和为：1 + 5 + 9 + 3 + 7 = 25
请注意，元素 mat[1][1] = 5 只会被计算一次。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> mat = [[<b>1</b>,1,1,<b>1</b>],
            [1,<b>1</b>,<b>1</b>,1],
            [1,<b>1</b>,<b>1</b>,1],
            [<b>1</b>,1,1,<b>1</b>]]
<b>输出:</b> 8
</pre>

#### 示例 3:
<pre>
<b>输入:</b> mat = [[<b>5</b>]]
<b>输出:</b> 5
</pre>

#### 提示:
* `n == mat.length == mat[i].length`
* `1 <= n <= 100`
* `1 <= mat[i][j] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let mut ret = 0;

        for i in 0..n {
            ret += mat[i][i] + mat[i][n - 1 - i];
        }

        if n % 2 == 1 {
            ret -= mat[n / 2][n / 2];
        }

        ret
    }
}
```
