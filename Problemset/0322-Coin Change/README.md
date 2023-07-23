# 322. Coin Change
You are given coins of different denominations and a total amount of money *amount*. Write a function to compute the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return ```-1```.

#### Example 1:
<pre>
<strong>Input:</strong> coins = [1, 2, 5], amount = 11
<strong>Output:</strong> 3
<strong>Explanation:</strong> 11 = 5 + 5 + 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> coins = [2], amount = 3
<strong>Output:</strong> -1
</pre>

#### Note:
You may assume that you have an infinite number of each kind of coin.

## Solutions (Rust)

### 1. Dynamic Programming
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
