# 1289. Minimum Falling Path Sum II
Given an `n x n` integer matrix `grid`, return *the minimum sum of a **falling path with non-zero shifts***.

A **falling path with non-zero shifts** is a choice of exactly one element from each row of `grid` such that no two elements chosen in adjacent rows are in the same column.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/10/falling-grid.jpg)
<pre>
<strong>Input:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> 13
<strong>Explanation:</strong>
The possible falling paths are:
[1,5,9], [1,5,7], [1,6,7], [1,6,8],
[2,4,8], [2,4,9], [2,6,7], [2,6,8],
[3,4,8], [3,4,9], [3,5,7], [3,5,9]
The falling path with the smallest sum is [1,5,7], so the answer is 13.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[7]]
<strong>Output:</strong> 7
</pre>

#### Constraints:
* `n == grid.length == grid[i].length`
* `1 <= n <= 200`
* `-99 <= grid[i][j] <= 99`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_falling_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 1 {
            return grid[0][0];
        }

        let mut min_col2 = [grid.len(); 2];
        let mut min_sum2 = [0; 2];

        for r in 0..grid.len() {
            let mut i = grid.len();
            let mut j = grid.len();

            for c in 0..grid.len() {
                if c != min_col2[0] {
                    grid[r][c] += min_sum2[0];
                } else {
                    grid[r][c] += min_sum2[1];
                }

                if grid[r][c] <= *grid[r].get(i).unwrap_or(&i32::MAX) {
                    j = i;
                    i = c;
                } else if grid[r][c] < *grid[r].get(j).unwrap_or(&i32::MAX) {
                    j = c;
                }
            }

            min_col2 = [i, j];
            min_sum2 = [grid[r][i], grid[r][j]];
        }

        min_sum2[0].min(min_sum2[1])
    }
}
```
