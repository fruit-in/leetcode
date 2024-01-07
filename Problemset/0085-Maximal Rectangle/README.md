# 85. Maximal Rectangle
Given a `rows x cols` binary `matrix` filled with `0`'s and `1`'s, find the largest rectangle containing only `1`'s and return *its area*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/14/maximal.jpg)
<pre>
<strong>Input:</strong> matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
<strong>Output:</strong> 6
<strong>Explanation:</strong> The maximal rectangle is shown in the above picture.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matrix = [["0"]]
<strong>Output:</strong> 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> matrix = [["1"]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `rows == matrix.length`
* `cols == matrix[i].length`
* `1 <= row, cols <= 200`
* `matrix[i][j]` is `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut prefix_sum = vec![vec![0; cols]; rows];
        let mut ret = 0;

        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == '0' {
                    continue;
                }

                prefix_sum[r][c] = 1;

                if c > 0 {
                    prefix_sum[r][c] += prefix_sum[r][c - 1];
                }

                let mut min_w = i32::MAX;

                for h in 1..=r + 1 {
                    if prefix_sum[r + 1 - h][c] == 0 {
                        break;
                    }

                    min_w = min_w.min(prefix_sum[r + 1 - h][c]);
                    ret = ret.max(min_w * h as i32);
                }
            }
        }

        ret
    }
}
```
