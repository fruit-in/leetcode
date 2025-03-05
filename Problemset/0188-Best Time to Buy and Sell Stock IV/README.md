# 188. Best Time to Buy and Sell Stock IV
You are given an integer array `prices` where `prices[i]` is the price of a given stock on the <code>i<sup>th</sup></code> day, and an integer `k`.

Find the maximum profit you can achieve. You may complete at most k transactions: i.e. you may buy at most `k` times and sell at most `k` times.

**Note:** You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

#### Example 1:
<pre>
<strong>Input:</strong> k = 2, prices = [2,4,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 2, prices = [3,2,6,5,0,3]
<strong>Output:</strong> 7
<strong>Explanation:</strong> Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4. Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
</pre>

#### Constraints:
* `1 <= k <= 100`
* `1 <= prices.length <= 1000`
* `0 <= prices[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut dp = vec![[0, i32::MIN]; k + 1];
        let mut ret = 0;

        for i in 0..prices.len() {
            let mut tmp = dp.clone();

            for j in 0..=i.min(k) {
                if j < k {
                    tmp[j + 1][1] = tmp[j + 1][1].max(dp[j][0] - prices[i]);
                }
                tmp[j][0] = tmp[j][0].max(dp[j][1] + prices[i]);
                ret = ret.max(tmp[j][0]);
            }

            dp = tmp;
        }

        ret
    }
}
```
