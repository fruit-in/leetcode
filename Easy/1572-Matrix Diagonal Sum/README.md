# 1572. Matrix Diagonal Sum
Given a square matrix `mat`, return the sum of the matrix diagonals.

Only include the sum of all the elements on the primary diagonal and all the elements on the secondary diagonal that are not part of the primary diagonal.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/08/14/sample_1911.png)
<pre>
<b>Input:</b> mat = [[<b>1</b>,2,<b>3</b>],
              [4,<b>5</b>,6],
              [<b>7</b>,8,<b>9</b>]]
<b>Output:</b> 25
<b>Explanation:</b> Diagonals sum: 1 + 5 + 9 + 3 + 7 = 25
Notice that element mat[1][1] = 5 is counted only once.
</pre>

#### Example 2:
<pre>
<b>Input:</b> mat = [[<b>1</b>,1,1,<b>1</b>],
              [1,<b>1</b>,<b>1</b>,1],
              [1,<b>1</b>,<b>1</b>,1],
              [<b>1</b>,1,1,<b>1</b>]]
<b>Output:</b> 8
</pre>

#### Example 3:
<pre>
<b>Input:</b> mat = [[<b>5</b>]]
<b>Output:</b> 5
</pre>

#### Constraints:
* `n == mat.length == mat[i].length`
* `1 <= n <= 100`
* `1 <= mat[i][j] <= 100`

## Solutions (Rust)

### 1. Solution
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
