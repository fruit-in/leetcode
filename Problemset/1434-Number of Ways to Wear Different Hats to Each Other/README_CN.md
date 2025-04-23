# 1434. 每个人戴不同帽子的方案数
总共有 `n` 个人和 `40` 种不同的帽子，帽子编号从 `1` 到 `40` 。

给你一个整数列表的列表 `hats` ，其中 `hats[i]` 是第 `i` 个人所有喜欢帽子的列表。

请你给每个人安排一顶他喜欢的帽子，确保每个人戴的帽子跟别人都不一样，并返回方案数。

由于答案可能很大，请返回它对 `10^9 + 7` 取余后的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> hats = [[3,4],[4,5],[5]]
<strong>输出:</strong> 1
<strong>解释:</strong> 给定条件下只有一种方法选择帽子。
第一个人选择帽子 3，第二个人选择帽子 4，最后一个人选择帽子 5。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> hats = [[3,5,1],[3,5]]
<strong>输出:</strong> 4
<strong>解释:</strong> 总共有 4 种安排帽子的方法：
(3,5)，(5,3)，(1,3) 和 (1,5)
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> hats = [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]]
<strong>输出:</strong> 24
<strong>解释:</strong> 每个人都可以从编号为 1 到 4 的帽子中选。
(1,2,3,4) 4 个帽子的排列方案数为 24 。
</pre>

#### 提示:
* `n == hats.length`
* `1 <= n <= 10`
* `1 <= hats[i].length <= 40`
* `1 <= hats[i][j] <= 40`
* `hats[i]` 包含一个数字互不相同的整数列表。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let mut dp = vec![vec![0; 1 << n]; 41];
        dp[0][0] = 1;

        for i in 1..=40 {
            dp[i] = dp[i - 1].clone();

            for j in 0..n {
                if !hats[j].contains(&(i as i32)) {
                    continue;
                }

                for k in 0..(1 << n) {
                    if (k >> j) & 1 == 0 {
                        dp[i][k | (1 << j)] = (dp[i][k | (1 << j)] + dp[i - 1][k]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[40][(1 << n) - 1]
    }
}
```
