# 123. Best Time to Buy and Sell Stock III
You are given an array `prices` where `prices[i]` is the price of a given stock on the <code>i<sup>th</sup></code> day.

Find the maximum profit you can achieve. You may complete **at most two transactions**.

**Note:** You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

#### Example 1:
<pre>
<strong>Input:</strong> prices = [3,3,5,0,0,3,1,4]
<strong>Output:</strong> 6
<strong>Explanation:</strong> Buy on day 4 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
Then buy on day 7 (price = 1) and sell on day 8 (price = 4), profit = 4-1 = 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> prices = [1,2,3,4,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are engaging multiple transactions at the same time. You must sell before buying again.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> prices = [7,6,4,3,1]
<strong>Output:</strong> 0
<strong>Explanation:</strong> In this case, no transaction is done, i.e. max profit = 0.
</pre>

#### Constraints:
* <code>1 <= prices.length <= 10<sup>5</sup></code>
* <code>0 <= prices[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_price = i32::MIN;
        let mut once_max = vec![0; prices.len()];
        let mut ret = 0;

        for i in 1..prices.len() {
            min_price = min_price.min(prices[i]);
            once_max[i] = (prices[i] - min_price).max(once_max[i - 1]);
        }
        for i in (0..prices.len()).rev() {
            max_price = max_price.max(prices[i]);
            ret = ret.max(max_price - prices[i] + once_max[i]);
        }

        ret
    }
}
```
