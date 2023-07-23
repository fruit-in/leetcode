# 1886. Determine Whether Matrix Can Be Obtained By Rotation
Given two `n x n` binary matrices `mat` and `target`, return `true` *if it is possible to make* `mat` *equal to* `target` *by **rotating*** `mat` *in **90-degree increments**, or* `false` otherwise.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/20/grid3.png)
<pre>
<strong>Input:</strong> mat = [[0,1],[1,0]], target = [[1,0],[0,1]]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can rotate mat 90 degrees clockwise to make mat equal target.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/05/20/grid4.png)
<pre>
<strong>Input:</strong> mat = [[0,1],[1,1]], target = [[1,0],[0,1]]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to make mat equal to target by rotating mat.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/05/26/grid4.png)
<pre>
<strong>Input:</strong> mat = [[0,0,0],[0,1,0],[1,1,1]], target = [[1,1,1],[0,1,0],[0,0,0]]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can rotate mat 90 degrees clockwise two times to make mat equal target.
</pre>

#### Constraints:
* `n == mat.length == target.length`
* `n == mat[i].length == target[i].length`
* `1 <= n <= 10`
* `mat[i][j]` and `target[i][j]` are either `0` or `1`.

## Solutions (Rust)

### 1. Solution
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
