# 2240. Number of Ways to Buy Pens and Pencils
You are given an integer `total` indicating the amount of money you have. You are also given two integers `cost1` and `cost2` indicating the price of a pen and pencil respectively. You can spend **part or all** of your money to buy multiple quantities (or none) of each kind of writing utensil.

Return *the **number of distinct ways** you can buy some number of pens and pencils*.

#### Example 1:
<pre>
<strong>Input:</strong> total = 20, cost1 = 10, cost2 = 5
<strong>Output:</strong> 9
<strong>Explanation:</strong> The price of a pen is 10 and the price of a pencil is 5.
- If you buy 0 pens, you can buy 0, 1, 2, 3, or 4 pencils.
- If you buy 1 pen, you can buy 0, 1, or 2 pencils.
- If you buy 2 pens, you cannot buy any pencils.
The total number of ways to buy pens and pencils is 5 + 3 + 1 = 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> total = 5, cost1 = 10, cost2 = 10
<strong>Output:</strong> 1
<strong>Explanation:</strong> The price of both pens and pencils are 10, which cost more than total, so you cannot buy any writing utensils. Therefore, there is only 1 way: buy 0 pens and 0 pencils.
</pre>

#### Constraints:
* <code>1 <= total, cost1, cost2 <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let expensive = cost1.max(cost2);
        let cheap = cost1.min(cost2);

        (0..=total)
            .step_by(expensive as usize)
            .map(|x| ((total - x) / cheap) as i64 + 1)
            .sum()
    }
}
```
