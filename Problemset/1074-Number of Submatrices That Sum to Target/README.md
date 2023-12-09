# 1074. Number of Submatrices That Sum to Target
Given a `matrix` and a `target`, return the number of non-empty submatrices that sum to target.

A submatrix `x1, y1, x2, y2` is the set of all cells `matrix[x][y]` with `x1 <= x <= x2` and `y1 <= y <= y2`.

Two submatrices `(x1, y1, x2, y2)` and `(x1', y1', x2', y2')` are different if they have some coordinate that is different: for example, if `x1 != x1'`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/02/mate1.jpg)
<pre>
<strong>Input:</strong> matrix = [[0,1,0],[1,1,1],[0,1,0]], target = 0
<strong>Output:</strong> 4
<strong>Explanation:</strong> The four 1x1 submatrices that only contain 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matrix = [[1,-1],[-1,1]], target = 0
<strong>Output:</strong> 5
<strong>Explanation:</strong> The two 1x2 submatrices, plus the two 2x1 submatrices, plus the 2x2 submatrix.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> matrix = [[904]], target = 0
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= matrix.length <= 100`
* `1 <= matrix[0].length <= 100`
* `-1000 <= matrix[i] <= 1000`
* `-10^8 <= target <= 10^8`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut matrix = matrix;
        let mut count = HashMap::new();
        let mut ret = 0;

        for x1 in 0..m {
            for x2 in x1..m {
                count.insert((x1, x2, 0), 1);
            }
        }

        for x2 in 0..m {
            for y2 in 0..n {
                if x2 > 0 {
                    matrix[x2][y2] += matrix[x2 - 1][y2];
                }
                if y2 > 0 {
                    matrix[x2][y2] += matrix[x2][y2 - 1];
                }
                if x2 > 0 && y2 > 0 {
                    matrix[x2][y2] -= matrix[x2 - 1][y2 - 1];
                }

                for x1 in 0..=x2 {
                    let mut diff = matrix[x2][y2];
                    if x1 > 0 {
                        diff -= matrix[x1 - 1][y2];
                    }
                    ret += *count.get(&(x1, x2, diff - target)).unwrap_or(&0);
                    *count.entry((x1, x2, diff)).or_insert(0) += 1;
                }
            }
        }

        ret
    }
}
```
