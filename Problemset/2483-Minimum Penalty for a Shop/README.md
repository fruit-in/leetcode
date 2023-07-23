# 2483. Minimum Penalty for a Shop
You are given the customer visit log of a shop represented by a **0-indexed** string `customers` consisting only of characters `'N'` and `'Y'`:

* if the <code>i<sup>th</sup></code> character is `'Y'`, it means that customers come at the <code>i<sup>th</sup></code> hour
* whereas `'N'` indicates that no customers come at the <code>i<sup>th</sup></code> hour.

If the shop closes at the <code>j<sup>th</sup></code> hour (`0 <= j <= n`), the **penalty** is calculated as follows:

* For every hour when the shop is open and no customers come, the penalty increases by `1`.
* For every hour when the shop is closed and customers come, the penalty increases by `1`.

Return *the **earliest** hour at which the shop must be closed to incur a **minimum** penalty*.

**Note** that if a shop closes at the <code>j<sup>th</sup></code> hour, it means the shop is closed at the hour `j`.

#### Example 1:
<pre>
<strong>Input:</strong> customers = "YYNY"
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- Closing the shop at the 0th hour incurs in 1+1+0+1 = 3 penalty.
- Closing the shop at the 1st hour incurs in 0+1+0+1 = 2 penalty.
- Closing the shop at the 2nd hour incurs in 0+0+0+1 = 1 penalty.
- Closing the shop at the 3rd hour incurs in 0+0+1+1 = 2 penalty.
- Closing the shop at the 4th hour incurs in 0+0+1+0 = 1 penalty.
Closing the shop at 2nd or 4th hour gives a minimum penalty. Since 2 is earlier, the optimal closing time is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> customers = "NNNNN"
<strong>Output:</strong> 0
<strong>Explanation:</strong> It is best to close the shop at the 0th hour as no customers arrive.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> customers = "YYYY"
<strong>Output:</strong> 4
<strong>Explanation:</strong> It is best to close the shop at the 4th hour as customers arrive at each hour.
</pre>

#### Constraints:
* <code>1 <= customers.length <= 10<sup>5</sup></code>
* `customers` consists only of characters `'Y'` and `'N'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut count_n = 0;
        let mut count_y = customers.chars().filter(|&c| c == 'Y').count();
        let mut min_penalty = count_n + count_y;
        let mut ret = 0;

        for (i, c) in customers.chars().enumerate() {
            if c == 'N' {
                count_n += 1;
            } else if c == 'Y' {
                count_y -= 1;
            }

            if min_penalty > count_n + count_y {
                min_penalty = count_n + count_y;
                ret = i + 1;
            }
        }

        ret as i32
    }
}
```
