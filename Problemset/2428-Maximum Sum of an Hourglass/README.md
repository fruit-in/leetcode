# 2428. Maximum Sum of an Hourglass
You are given an `m x n` integer matrix `grid`.

We define an **hourglass** as a part of the matrix with the following form:

![](https://assets.leetcode.com/uploads/2022/08/21/img.jpg)

Return *the **maximum** sum of the elements of an hourglass*.

**Note** that an hourglass cannot be rotated and must be entirely contained within the matrix.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/08/21/1.jpg)
<pre>
<strong>Input:</strong> grid = [[6,2,1,3],[4,2,1,5],[9,2,8,7],[4,1,2,9]]
<strong>Output:</strong> 30
<strong>Explanation:</strong> The cells shown above represent the hourglass with the maximum sum: 6 + 2 + 1 + 2 + 9 + 2 + 8 = 30.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/08/21/2.jpg)
<pre>
<strong>Input:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> 35
<strong>Explanation:</strong> There is only one hourglass in the matrix, with the sum: 1 + 2 + 3 + 5 + 7 + 8 + 9 = 35.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `3 <= m, n <= 150`
* <code>0 <= grid[i][j] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                ret = ret.max(
                    grid[i][j]
                        + grid[i - 1][j]
                        + grid[i + 1][j]
                        + grid[i - 1][j - 1]
                        + grid[i + 1][j - 1]
                        + grid[i - 1][j + 1]
                        + grid[i + 1][j + 1],
                );
            }
        }

        ret
    }
}
```
