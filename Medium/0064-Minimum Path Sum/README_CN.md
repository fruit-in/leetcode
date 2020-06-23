# 64. 最小路径和
给定一个包含非负整数的 *m* x *n* 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

**说明:** 每次只能向下或者向右移动一步。

#### 示例:
<pre>
<strong>输入:</strong>
[
  [1,3,1],
  [1,5,1],
  [4,2,1]
]
<strong>输出:</strong> 7
<strong>解释:</strong> 因为路径 1→3→1→1→1 的总和最小。
</pre>

## 题解 (Rust)

### 1. 动态规划
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
