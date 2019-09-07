# 122. Best Time to Buy and Sell Stock II
Say you have an array for which the *i*<sup>th</sup> element is the price of a given stock on day *i*.

Design an algorithm to find the maximum profit. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times).

**Note:** You may not engage in multiple transactions at the same time (i.e., you must sell the stock before you buy again).

#### Example 1:
<pre>
<strong>Input:</strong> [7,1,5,3,6,4]
<strong>Output:</strong> 7
<strong>Explanation:</strong> Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
             Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,3,4,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
             Note that you cannot buy on day 1, buy on day 2 and sell them later,
             as you are engaging multiple transactions at the same time.
             You must sell before buying again.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [7,6,4,3,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> In this case, no transaction is done, i.e. max profit = 0.
</pre>

## Solutions (Rust)

### 1. Add the Positive Difference between the Consecutive Numbers
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        for i in 1..prices.len() {
            profit += 0.max(prices[i] - prices[i - 1]);
        }
        profit
    }
}
```

### 2. Peak & Valley
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut i = 0;
        while i + 1 < prices.len() {
            while i + 1 < prices.len() && prices[i] >= prices[i + 1] {
                i += 1;
            }
            profit -= prices[i];
            while i + 1 < prices.len() && prices[i] <= prices[i + 1] {
                i += 1;
            }
            profit += prices[i];
        }
        profit
    }
}
```
