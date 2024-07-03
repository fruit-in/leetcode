# 309. Best Time to Buy and Sell Stock with Cooldown
You are given an array `prices` where `prices[i]` is the price of a given stock on the <code>i<sup>th</sup></code> day.

Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:

* After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).

**Note:** You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

#### Example 1:
<pre>
<strong>Input:</strong> prices = [1,2,3,0,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> transactions = [buy, sell, cooldown, buy, sell]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> prices = [1]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= prices.length <= 5000`
* `0 <= prices[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![(0, 0, i32::MIN, 0); n];
        dp[0].0 = -prices[0];

        for i in 1..n {
            dp[i].0 = dp[i - 1].3 - prices[i];
            dp[i].2 = dp[i - 1].0.max(dp[i - 1].2);
            dp[i].1 = 0.max(dp[i].2 + prices[i]);
            dp[i].3 = dp[i - 1].1.max(dp[i - 1].3);
        }

        dp[n - 1].1.max(dp[n - 1].3)
    }
}
```
