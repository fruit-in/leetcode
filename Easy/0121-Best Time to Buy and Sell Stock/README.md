# 121. Best Time to Buy and Sell Stock
Say you have an array for which the *i*<sup>th</sup> element is the price of a given stock on day *i*.

If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.

Note that you cannot sell a stock before you buy one.

#### Example 1:
<pre>
<strong>Input:</strong> [7,1,5,3,6,4]
<strong>Output:</strong> 5
<strong>Explanation:</strong> Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
             Not 7-1 = 6, as selling price needs to be larger than buying price.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [7,6,4,3,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> In this case, no transaction is done, i.e. max profit = 0.
</pre>

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        for (i, buy) in prices.iter().enumerate() {
            for sell in &prices[(i + 1)..] {
                if sell - buy > max_profit {
                    max_profit = sell - buy;
                }
            }
        }
        max_profit
    }
}
```

### 2. One Pass
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = std::i32::MAX;
        for n in prices {
            max_profit = max_profit.max(n - min_price);
            min_price = min_price.min(n);
        }
        max_profit
    }
}
```
