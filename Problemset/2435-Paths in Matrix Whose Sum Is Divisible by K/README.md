# 2435. Paths in Matrix Whose Sum Is Divisible by K
You are given a **0-indexed** `m x n` integer matrix `grid` and an integer `k`. You are currently at position `(0, 0)` and you want to reach position `(m - 1, n - 1)` moving only **down** or **right**.

Return *the number of paths where the sum of the elements on the path is divisible by* `k`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/08/13/image-20220813183124-1.png)
<pre>
<strong>Input:</strong> grid = [[5,2,4],[3,0,5],[0,7,2]], k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two paths where the sum of the elements on the path is divisible by k.
The first path highlighted in red has a sum of 5 + 2 + 4 + 5 + 2 = 18 which is divisible by 3.
The second path highlighted in blue has a sum of 5 + 3 + 0 + 5 + 2 = 15 which is divisible by 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/08/17/image-20220817112930-3.png)
<pre>
<strong>Input:</strong> grid = [[0,0]], k = 5
<strong>Output:</strong> 1
<strong>Explanation:</strong> The path highlighted in red has a sum of 0 + 0 = 0 which is divisible by 5.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2022/08/12/image-20220812224605-3.png)
<pre>
<strong>Input:</strong> grid = [[7,3,4,9],[2,3,6,2],[2,3,7,0]], k = 1
<strong>Output:</strong> 10
<strong>Explanation:</strong> Every integer is divisible by 1 so the sum of the elements on every possible path is divisible by k.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 5 * 10<sup>4</sup></code>
* <code>1 <= m * n <= 5 * 10<sup>4</sup></code>
* `0 <= grid[i][j] <= 100`
* `1 <= k <= 50`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let mut prev_row_mod = vec![VecDeque::from(vec![0; k]); n];
        prev_row_mod[0][0] = 1;

        for r in 0..m {
            let mut row_mod = prev_row_mod.clone();
            row_mod[0].rotate_right(grid[r][0] as usize % k);

            for c in 1..n {
                for i in 0..k {
                    row_mod[c][i] = (row_mod[c][i] + row_mod[c - 1][i]) % 1_000_000_007;
                }
                row_mod[c].rotate_right(grid[r][c] as usize % k);
            }

            prev_row_mod = row_mod;
        }

        prev_row_mod[n - 1][0]
    }
}
```
