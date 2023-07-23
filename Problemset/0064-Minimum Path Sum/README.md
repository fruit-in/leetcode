# 64. Minimum Path Sum
Given a *m* x *n* grid filled with non-negative numbers, find a path from top left to bottom right which *minimizes* the sum of all numbers along its path.

**Note:** You can only move either down or right at any point in time.

#### Example:
<pre>
<strong>Input:</strong>
[
  [1,3,1],
  [1,5,1],
  [4,2,1]
]
<strong>Output:</strong> 7
<strong>Explanation:</strong> Because the path 1→3→1→1→1 minimizes the sum.
</pre>

## Solutions (Ruby)

### 1. Dynamic Programming
```Ruby
# @param {Integer[][]} grid
# @return {Integer}
def min_path_sum(grid)
    m, n = grid.length, grid[0].length

    for i in 0...m
        for j in 0...n
            if i == 0 and j != 0
                grid[i][j] += grid[i][j - 1]
            elsif i != 0 and j == 0
                grid[i][j] += grid[i - 1][j]
            elsif i != 0 and j != 0
                grid[i][j] += [grid[i][j - 1], grid[i - 1][j]].min
            end
        end
    end

    return grid[m - 1][n - 1]
end
```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = grid.clone();

        for i in 0..m {
            for j in 0..n {
                dp[i][j] += match (i, j) {
                    (0, 0) => 0,
                    (0, _) => dp[i][j - 1],
                    (_, 0) => dp[i - 1][j],
                    _ => dp[i][j - 1].min(dp[i - 1][j]),
                };
            }
        }

        dp[m - 1][n - 1]
    }
}
```
