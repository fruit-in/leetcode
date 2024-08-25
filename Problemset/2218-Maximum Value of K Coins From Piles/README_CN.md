# 2218. 从栈中取出 K 个硬币的最大面值和
一张桌子上总共有 `n` 个硬币 **栈** 。每个栈有 **正整数** 个带面值的硬币。

每一次操作中，你可以从任意一个栈的 **顶部** 取出 1 个硬币，从栈中移除它，并放入你的钱包里。

给你一个列表 `piles` ，其中 `piles[i]` 是一个整数数组，分别表示第 `i` 个栈里 **从顶到底** 的硬币面值。同时给你一个正整数 `k` ，请你返回在 **恰好** 进行 `k` 次操作的前提下，你钱包里硬币面值之和 **最大为多少** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/11/09/e1.png)
<pre>
<strong>输入:</strong> piles = [[1,100,3],[7,8,9]], k = 2
<strong>输出:</strong> 101
<strong>解释:</strong>
上图展示了几种选择 k 个硬币的不同方法。
我们可以得到的最大面值为 101 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> piles = [[100],[100],[100],[100],[100],[100],[1,1,1,1,1,1,700]], k = 7
<strong>输出:</strong> 706
<strong>解释:</strong>
如果我们所有硬币都从最后一个栈中取，可以得到最大面值和。
</pre>

#### 提示:
* `n == piles.length`
* `1 <= n <= 1000`
* <code>1 <= piles[i][j] <= 10<sup>5</sup></code>
* `1 <= k <= sum(piles[i].length) <= 2000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let n = piles.len();
        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 0..n {
            for j in 0..=k {
                let mut total = 0;

                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                for m in 0..piles[i].len().min(k - j) {
                    total += piles[i][m];
                    dp[i + 1][j + m + 1] = dp[i + 1][j + m + 1].max(total + dp[i][j]);
                }
            }
        }

        dp[n][k]
    }
}
```
