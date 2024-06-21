# 714. Best Time to Buy and Sell Stock with Transaction Fee
You are given an array `prices` where `prices[i]` is the price of a given stock on the <code>i<sup>th</sup></code> day, and an integer `fee` representing a transaction fee.

Find the maximum profit you can achieve. You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction.

Note:

* You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
* The transaction fee is only charged once for each stock purchase and sale.

#### Example 1:
<pre>
<strong>Input:</strong> prices = [1,3,2,8,4,9], fee = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong> The maximum profit can be achieved by:
- Buying at prices[0] = 1
- Selling at prices[3] = 8
- Buying at prices[4] = 4
- Selling at prices[5] = 9
The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> prices = [1,3,7,5,10,3], fee = 3
<strong>Output:</strong> 6
</pre>

#### Constraints:
* <code>1 <= prices.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= prices[i] < 5 * 10<sup>4</sup></code>
* <code>0 <= fee < 5 * 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut x, mut y) = (0, -prices[0] - fee);

        for i in 1..prices.len() {
            (x, y) = (x.max(y + prices[i]), y.max(x - prices[i] - fee));
        }

        x
    }
}
```
