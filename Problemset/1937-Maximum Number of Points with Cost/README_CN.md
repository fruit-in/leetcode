# 1937. 扣分后的最大得分
给你一个 `m x n` 的整数矩阵 `points` （下标从 **0** 开始）。一开始你的得分为 `0` ，你想最大化从矩阵中得到的分数。

你的得分方式为：**每一行** 中选取一个格子，选中坐标为 `(r, c)` 的格子会给你的总得分 **增加** `points[r][c]` 。

然而，相邻行之间被选中的格子如果隔得太远，你会失去一些得分。对于相邻行 `r` 和 `r + 1` （其中 `0 <= r < m - 1`），选中坐标为 <code>(r, c<sub>1</sub>)</code> 和 <code>(r + 1, c<sub>2</sub>)</code> 的格子，你的总得分 **减少** <code>abs(c<sub>1</sub> - c<sub>2</sub>)</code> 。

请你返回你能得到的 **最大** 得分。

`abs(x)` 定义为：
* 如果 `x >= 0` ，那么值为 `x` 。
* 如果 `x < 0` ，那么值为 `-x` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/07/12/screenshot-2021-07-12-at-13-40-26-diagram-drawio-diagrams-net.png)
<pre>
<strong>输入:</strong> points = [[1,2,3],[1,5,1],[3,1,1]]
<strong>输出:</strong> 9
<strong>解释:</strong>
蓝色格子是最优方案选中的格子，坐标分别为 (0, 2)，(1, 1) 和 (2, 0) 。
你的总得分增加 3 + 5 + 3 = 11 。
但是你的总得分需要扣除 abs(2 - 1) + abs(1 - 0) = 2 。
你的最终得分为 11 - 2 = 9 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/07/12/screenshot-2021-07-12-at-13-42-14-diagram-drawio-diagrams-net.png)
<pre>
<strong>输入:</strong> points = [[1,5],[2,3],[4,2]]
<strong>输出:</strong> 11
<strong>解释:</strong>
蓝色格子是最优方案选中的格子，坐标分别为 (0, 1)，(1, 1) 和 (2, 0) 。
你的总得分增加 5 + 3 + 4 = 12 。
但是你的总得分需要扣除 abs(1 - 1) + abs(1 - 0) = 1 。
你的最终得分为 12 - 1 = 11 。
</pre>

#### 提示:
* `m == points.length`
* `n == points[r].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* <code>0 <= points[r][c] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (points.len(), points[0].len());
        let mut dp = vec![0; n];

        for r in 0..m {
            let mut tmp = vec![0; n];
            let (mut x, mut i) = (dp[0], 0);
            let (mut y, mut j) = (dp[n - 1], n - 1);

            for (c1, c2) in (0..n).zip((0..n).rev()) {
                if x - dp[c1] < (c1 - i) as i64 {
                    x = dp[c1];
                    i = c1;
                }
                if y - dp[c2] < (j - c2) as i64 {
                    y = dp[c2];
                    j = c2;
                }

                tmp[c1] = tmp[c1].max(points[r][c1] as i64 + x - (c1 - i) as i64);
                tmp[c2] = tmp[c2].max(points[r][c2] as i64 + y - (j - c2) as i64);
            }

            dp = tmp;
        }

        *dp.iter().max().unwrap()
    }
}
```
