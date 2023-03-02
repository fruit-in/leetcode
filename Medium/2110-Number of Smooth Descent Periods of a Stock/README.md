# 2110. Number of Smooth Descent Periods of a Stock
You are given an integer array `prices` representing the daily price history of a stock, where `prices[i]` is the stock price on the <code>i<sup>th</sup></code> day.

A **smooth descent period** of a stock consists of **one or more contiguous** days such that the price on each day is **lower** than the price on the **preceding day** by **exactly** `1`. The first day of the period is exempted from this rule.

Return *the number of **smooth descent periods***.

#### Example 1:
<pre>
<strong>Input:</strong> prices = [3,2,1,4]
<strong>Output:</strong> 7
<strong>Explanation:</strong> There are 7 smooth descent periods:
[3], [2], [1], [4], [3,2], [2,1], and [3,2,1]
Note that a period with one day is a smooth descent period by the definition.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> prices = [8,6,7,7]
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 smooth descent periods: [8], [6], [7], and [7]
Note that [8,6] is not a smooth descent period as 8 - 6 ≠ 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> prices = [1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is 1 smooth descent period: [1]
</pre>

#### Constraints:
* <code>1 <= prices.length <= 10<sup>5</sup></code>
* <code>1 <= prices[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut count = 1;
        let mut ret = 0;

        for i in 1..prices.len() {
            if prices[i] - prices[i - 1] == -1 {
                count += 1;
            } else {
                ret += count * (count + 1) / 2;
                count = 1;
            }
        }

        ret += count * (count + 1) / 2;

        ret
    }
}
```
