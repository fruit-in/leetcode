# 1648. Sell Diminishing-Valued Colored Balls
You have an `inventory` of different colored balls, and there is a customer that wants `orders` balls of **any** color.

The customer weirdly values the colored balls. Each colored ball's value is the number of balls **of that color** you currently have in your `inventory`. For example, if you own `6` yellow balls, the customer would pay `6` for the first yellow ball. After the transaction, there are only `5` yellow balls left, so the next yellow ball is then valued at `5` (i.e., the value of the balls decreases as you sell more to the customer).

You are given an integer array, `inventory`, where `inventory[i]` represents the number of balls of the <code>i<sup>th</sup></code> color that you initially own. You are also given an integer `orders`, which represents the total number of balls that the customer wants. You can sell the balls **in any order**.

Return *the **maximum** total value that you can attain after selling* `orders` *colored balls*. As the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/11/05/jj.gif)
<pre>
<strong>Input:</strong> inventory = [2,5], orders = 4
<strong>Output:</strong> 14
<strong>Explanation:</strong> Sell the 1st color 1 time (2) and the 2nd color 3 times (5 + 4 + 3).
The maximum total value is 2 + 5 + 4 + 3 = 14.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> inventory = [3,5], orders = 6
<strong>Output:</strong> 19
<strong>Explanation:</strong> Sell the 1st color 2 times (3 + 2) and the 2nd color 4 times (5 + 4 + 3 + 2).
The maximum total value is 3 + 2 + 5 + 4 + 3 + 2 = 19.
</pre>

#### Constraints:
* <code>1 <= inventory.length <= 10<sup>5</sup></code>
* <code>1 <= inventory[i] <= 10<sup>9</sup></code>
* <code>1 <= orders <= min(sum(inventory[i]), 10<sup>9</sup>)</code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        let mut orders = orders as i64;
        let mut count = HashMap::new();
        let mut ret = 0;

        for &balls in &inventory {
            count
                .entry(balls as i64)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }

        let mut heap = count.into_iter().collect::<BinaryHeap<_>>();

        while orders > 0 {
            let (b0, c0) = heap.pop().unwrap();
            let (b1, c1) = heap.pop().unwrap_or((0, 0));

            if (b0 - b1) * c0 < orders {
                orders -= (b0 - b1) * c0;
                heap.push((b1, c0 + c1));
                ret = (ret + (b0 - b1) * (b0 + b1 + 1) * c0 / 2) % 1_000_000_007;
            } else {
                let (x, y) = (orders / c0, orders % c0);
                orders = 0;
                ret = (ret + (b0 * 2 - x + 1) * x * c0 / 2 + (b0 - x) * y) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
```
