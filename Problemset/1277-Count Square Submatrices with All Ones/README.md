# 1277. Count Square Submatrices with All Ones
Given a `m * n` matrix of ones and zeros, return how many **square** submatrices have all ones.

#### Example 1:
<pre>
<strong>Input:</strong> matrix =
[
  [0,1,1,1],
  [1,1,1,1],
  [0,1,1,1]
]
<strong>Output:</strong> 15
<strong>Explanation:</strong>
There are <b>10</b> squares of side 1.
There are <b>4</b> squares of side 2.
There is  <b>1</b> square of side 3.
Total number of squares = 10 + 4 + 1 = <b>15</b>.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matrix =
[
  [1,0,1],
  [1,1,0],
  [1,1,0]
]
<strong>Output:</strong> 7
<strong>Explanation:</strong>
There are <b>6</b> squares of side 1.
There is <b>1</b> square of side 2.
Total number of squares = 6 + 1 = <b>7</b>.
</pre>

#### Constraints:
* `1 <= arr.length <= 300`
* `1 <= arr[0].length <= 300`
* `0 <= arr[i][j] <= 1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 1 && i > 0 && j > 0 {
                    matrix[i][j] +=
                        matrix[i - 1][j - 1].min(matrix[i][j - 1].min(matrix[i - 1][j]));
                }

                ret += matrix[i][j];
            }
        }

        ret
    }
}
```
