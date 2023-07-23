# 322. 零钱兑换
给定不同面额的硬币 coins 和一个总金额 amount。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回 ```-1```。

#### 示例 1:
<pre>
<strong>输入:</strong> coins = [1, 2, 5], amount = 11
<strong>输出:</strong> 3
<strong>解释:</strong> 11 = 5 + 5 + 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> coins = [2], amount = 3
<strong>输出:</strong> -1
</pre>

#### 说明:
你可以认为每种硬币的数量是无限的。

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![-1; amount as usize + 1];
        dp[0] = 0;

        for &coin in &coins {
            for i in (coin as usize)..=(amount as usize) {
                dp[i] = match dp[i - coin as usize] {
                    x if x != -1 && (dp[i] == -1 || dp[i] > x + 1) => x + 1,
                    _ => dp[i],
                };
            }
        }

        dp[amount as usize]
    }
}
```
