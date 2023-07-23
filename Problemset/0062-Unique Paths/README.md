# 62. Unique Paths
A robot is located at the top-left corner of a *m* x *n* grid (marked 'Start' in the diagram below).

The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).

How many possible unique paths are there?

![](https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png)<br>
Above is a 7 x 3 grid. How many possible unique paths are there?

#### Example 1:
<pre>
<strong>Input:</strong> m = 3, n = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong>
From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
1. Right -> Right -> Down
2. Right -> Down -> Right
3. Down -> Right -> Right
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> m = 7, n = 3
<strong>Output:</strong> 28
</pre>

#### Constraints:
* `1 <= m, n <= 100`
* It's guaranteed that the answer will be less than or equal to `2 * 10 ^ 9`.

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n]; m];
        dp[m - 1][n - 1] = 1;

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i < m - 1 {
                    dp[i][j] += dp[i + 1][j];
                }
                if j < n - 1 {
                    dp[i][j] += dp[i][j + 1];
                }
            }
        }

        dp[0][0]
    }
}
```

### 2. Mathematical
```Rust
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        (1..(m as i64)).fold(1, |acc, x| acc * (n as i64 - 1 + x) / x) as i32
    }
}
```
