# 518. Coin Change 2
You are given coins of different denominations and a total amount of money. Write a function to compute the number of combinations that make up that amount. You may assume that you have infinite number of each kind of coin.

#### Example 1:
<pre>
<strong>Input:</strong> amount = 5, coins = [1, 2, 5]
<strong>Output:</strong> 4
<strong>Explanation:</strong> there are four ways to make up the amount:
5=5
5=2+2+1
5=2+1+1+1
5=1+1+1+1+1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> amount = 3, coins = [2]
<strong>Output:</strong> 0
<strong>Explanation:</strong> the amount of 3 cannot be made up just with coins of 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> amount = 10, coins = [10]
<strong>Output:</strong> 1
</pre>

#### Note:
You can assume that
* 0 <= amount <= 5000
* 1 <= coin <= 5000
* the number of coins is less than 500
* the answer is guaranteed to fit into signed 32-bit integer

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for &coin in &coins {
            for i in (coin as usize)..=(amount as usize) {
                dp[i] += dp[i - coin as usize];
            }
        }

        dp[amount as usize]
    }
}
```
