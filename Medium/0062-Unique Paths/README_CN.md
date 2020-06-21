# 62. 不同路径
一个机器人位于一个 *m* x *n* 网格的左上角 （起始点在下图中标记为“Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。

问总共有多少条不同的路径？

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/robot_maze.png)<br>
例如，上图是一个7 x 3 的网格。有多少可能的路径？

#### 示例 1:
<pre>
<strong>输入:</strong> m = 3, n = 2
<strong>输出:</strong> 3
<strong>解释:</strong>
从左上角开始，总共有 3 条路径可以到达右下角。
1. 向右 -> 向右 -> 向下
2. 向右 -> 向下 -> 向右
3. 向下 -> 向右 -> 向右
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> m = 7, n = 3
<strong>输出:</strong> 28
</pre>

#### 提示:
* `1 <= m, n <= 100`
* 题目数据保证答案小于等于 `2 * 10 ^ 9`

## 题解 (Rust)

### 1. 动态规划
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

### 2. 数学
```Rust
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        (1..(m as i64)).fold(1, |acc, x| acc * (n as i64 - 1 + x) / x) as i32
    }
}
```
