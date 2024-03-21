# 1473. 粉刷房子 III
在一个小城市里，有 `m` 个房子排成一排，你需要给每个房子涂上 `n` 种颜色之一（颜色编号为 `1` 到 `n` ）。有的房子去年夏天已经涂过颜色了，所以这些房子不可以被重新涂色。

我们将连续相同颜色尽可能多的房子称为一个街区。（比方说 `houses = [1,2,2,3,3,2,1,1]` ，它包含 5 个街区  `[{1}, {2,2}, {3,3}, {2}, {1,1}]` 。）

给你一个数组 `houses` ，一个 `m * n` 的矩阵 `cost` 和一个整数 `target` ，其中：

* `houses[i]`：是第 `i` 个房子的颜色，**0** 表示这个房子还没有被涂色。
* `cost[i][j]`：是将第 `i` 个房子涂成颜色 `j+1` 的花费。

请你返回房子涂色方案的最小总花费，使得每个房子都被涂色后，恰好组成 `target` 个街区。如果没有可用的涂色方案，请返回 **-1** 。

#### 示例 1:
<pre>
<strong>输入:</strong> houses = [0,0,0,0,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
<strong>输出:</strong> 9
<strong>解释:</strong> 房子涂色方案为 [1,2,2,1,1]
此方案包含 target = 3 个街区，分别是 [{1}, {2,2}, {1,1}]。
涂色的总花费为 (1 + 1 + 1 + 1 + 5) = 9。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> houses = [0,2,1,2,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
<strong>输出:</strong> 11
<strong>解释:</strong> 有的房子已经被涂色了，在此基础上涂色方案为 [2,2,1,2,2]
此方案包含 target = 3 个街区，分别是 [{2,2}, {1}, {2,2}]。
给第一个和最后一个房子涂色的花费为 (10 + 1) = 11。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> houses = [0,0,0,0,0], cost = [[1,10],[10,1],[1,10],[10,1],[1,10]], m = 5, n = 2, target = 5
<strong>输出:</strong> 5
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> houses = [3,1,2,3], cost = [[1,1,1],[1,1,1],[1,1,1],[1,1,1]], m = 4, n = 3, target = 3
<strong>输出:</strong> -1
<strong>解释:</strong> 房子已经被涂色并组成了 4 个街区，分别是 [{3},{1},{2},{3}] ，无法形成 target = 3 个街区。
</pre>

#### 提示:
* `m == houses.length == cost.length`
* `n == cost[i].length`
* `1 <= m <= 100`
* `1 <= n <= 20`
* `1 <= target <= m`
* `0 <= houses[i] <= n`
* <code>1 <= cost[i][j] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (m, n, target) = (m as usize, n as usize, target as usize);
        let houses = houses.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut dp = vec![vec![vec![None; n]; target + 1]; m];

        if houses[0] > 0 {
            dp[0][1][houses[0] as usize - 1] = Some(0);
        } else {
            for j in 0..n {
                dp[0][1][j] = Some(cost[0][j]);
            }
        }

        for i in 1..m {
            for k in 1..=target {
                for j in 0..n {
                    if let Some(x) = dp[i - 1][k][j] {
                        if houses[i] > 0 {
                            if houses[i] - 1 == j {
                                dp[i][k][j] = Some(dp[i][k][j].unwrap_or(i32::MAX).min(x));
                            } else if k + 1 <= target {
                                dp[i][k + 1][houses[i] - 1] =
                                    Some(dp[i][k + 1][houses[i] - 1].unwrap_or(i32::MAX).min(x));
                            }
                        } else {
                            for jj in 0..n {
                                if jj == j {
                                    dp[i][k][jj] =
                                        Some(dp[i][k][jj].unwrap_or(i32::MAX).min(x + cost[i][jj]));
                                } else if k + 1 <= target {
                                    dp[i][k + 1][jj] = Some(
                                        dp[i][k + 1][jj].unwrap_or(i32::MAX).min(x + cost[i][jj]),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        dp[m - 1][target]
            .iter()
            .filter_map(|&x| x)
            .min()
            .unwrap_or(-1)
    }
}
```
