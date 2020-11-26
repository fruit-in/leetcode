# 221. Maximal Square
Given an `m x n` binary `matrix` filled with `0`'s and `1`'s, *find the largest square containing only* `1`'s *and return its area*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/11/26/max1grid.jpg)
<pre>
<strong>Input:</strong> matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
<strong>Output:</strong> 4
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/11/26/max2grid.jpg)
<pre>
<strong>Input:</strong> matrix = [["0","1"],["1","0"]]
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> matrix = [["0"]]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `m == matrix.length`
* `n == matrix[i].length`
* `1 <= m, n <= 300`
* `matrix[i][j]` is `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let mut matrix = matrix
            .iter()
            .map(|row| row.iter().map(|&c| c as i32 - 48).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] += matrix[i - 1][j - 1]
                        .min(matrix[i][j - 1])
                        .min(matrix[i - 1][j]);
                }
                ret = ret.max(matrix[i][j]);
            }
        }

        ret * ret
    }
}
```
