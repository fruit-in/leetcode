# 63. 不同路径 II
一个机器人位于一个 *m* x *n* 网格的左上角 （起始点在下图中标记为“Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。

现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/robot_maze.png)

网格中的障碍物和空位置分别用 `1` 和 `0` 来表示。

**说明:** *m* 和 *n* 的值均不超过 100。

#### 示例 1:
<pre>
<strong>输入:</strong>
[
  [0,0,0],
  [0,1,0],
  [0,0,0]
]
<strong>输出:</strong> 2
<strong>解释:</strong>
3x3 网格的正中间有一个障碍物。
从左上角到右下角一共有 2 条不同的路径：
1. 向右 -> 向右 -> 向下 -> 向下
2. 向下 -> 向下 -> 向右 -> 向右
</pre>

## 题解 (Rust)

### 1. 动态规划
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
