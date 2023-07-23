# 2133. Check if Every Row and Column Contains All Numbers
An `n x n` matrix is **valid** if every row and every column contains **all** the integers from `1` to `n` (**inclusive**).

Given an `n x n` integer matrix `matrix`, return `true` *if the matrix is **valid***. Otherwise, return `false`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/21/example1drawio.png)
<pre>
<strong>Input:</strong> matrix = [[1,2,3],[3,1,2],[2,3,1]]
<strong>Output:</strong> true
<strong>Explanation:</strong> In this case, n = 3, and every row and column contains the numbers 1, 2, and 3.
Hence, we return true.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/21/example2drawio.png)
<pre>
<strong>Input:</strong> matrix = [[1,1,1],[1,2,3],[1,2,3]]
<strong>Output:</strong> false
<strong>Explanation:</strong> In this case, n = 3, but the first row and the first column do not contain the numbers 2 or 3.
Hence, we return false.
</pre>

#### Constraints:
* `n == matrix.length == matrix[i].length`
* `1 <= n <= 100`
* `1 <= matrix[i][j] <= n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();

        for i in 0..n {
            let mut valid_r = vec![false; n];
            let mut valid_c = vec![false; n];

            for j in 0..n {
                valid_r[matrix[i][j] as usize - 1] = true;
                valid_c[matrix[j][i] as usize - 1] = true;
            }

            if !(0..n).all(|k| valid_r[k] && valid_c[k]) {
                return false;
            }
        }

        true
    }
}
```
