# 63. Unique Paths II
A robot is located at the top-left corner of a *m* x *n* grid (marked 'Start' in the diagram below).

The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).

Now consider if some obstacles are added to the grids. How many unique paths would there be?

![](https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png)

An obstacle and empty space is marked as `1` and `0` respectively in the grid.

**Note:** *m* and *n* will be at most 100.

#### Example 1:
<pre>
<strong>Input:</strong>
[
  [0,0,0],
  [0,1,0],
  [0,0,0]
]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
There is one obstacle in the middle of the 3x3 grid above.
There are two ways to reach the bottom-right corner:
1. Right -> Right -> Down -> Down
2. Down -> Down -> Right -> Right
</pre>

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut dp = vec![vec![0; n]; m];
        dp[m - 1][n - 1] = 1 - obstacle_grid[m - 1][n - 1];

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if obstacle_grid[i][j] == 0 {
                    if i < m - 1 {
                        dp[i][j] += dp[i + 1][j];
                    }
                    if j < n - 1 {
                        dp[i][j] += dp[i][j + 1];
                    }
                }
            }
        }

        dp[0][0]
    }
}
```
