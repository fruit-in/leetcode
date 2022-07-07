# 1886. 判断矩阵经轮转后是否一致
给你两个大小为 `n x n` 的二进制矩阵 `mat` 和 `target` 。现 **以 90 度顺时针轮转** 矩阵 `mat` 中的元素 **若干次** ，如果能够使 `mat` 与 `target` 一致，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/05/20/grid3.png)
<pre>
<strong>输入:</strong> mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
<strong>输出:</strong> true
<strong>解释:</strong> 顺时针轮转 90 度一次可以使 mat 和 target 一致。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/05/20/grid4.png)
<pre>
<strong>输入:</strong> mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
<strong>输出:</strong> false
<strong>解释:</strong> 无法通过轮转矩阵中的元素使 equal 与 target 一致。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/05/26/grid4.png)
<pre>
<strong>输入:</strong> mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
<strong>输出:</strong> true
<strong>解释:</strong> 顺时针轮转 90 度两次可以使 mat 和 target 一致。
</pre>

#### 提示:
* `n == mat.length == target.length`
* `n == mat[i].length == target[i].length`
* `1 <= n <= 10`
* `mat[i][j]` 和 `target[i][j]` 不是 `0` 就是 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut r0 = true;
        let mut r1 = true;
        let mut r2 = true;
        let mut r3 = true;
        let n = mat.len();

        for r in 0..n {
            for c in 0..n {
                r0 &= mat[r][c] == target[r][c];
                r1 &= mat[r][c] == target[c][n - 1 - r];
                r2 &= mat[r][c] == target[n - 1 - r][n - 1 - c];
                r3 &= mat[r][c] == target[n - 1 - c][r];
            }
        }

        r0 || r1 || r2 || r3
    }
}
```
