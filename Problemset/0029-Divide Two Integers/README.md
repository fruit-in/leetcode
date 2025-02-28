# 29. Divide Two Integers
Given two integers `dividend` and `divisor`, divide two integers **without** using multiplication, division, and mod operator.

The integer division should truncate toward zero, which means losing its fractional part. For example, `8.345` would be truncated to `8`, and `-2.7335` would be truncated to `-2`.

Return *the **quotient** after dividing* `dividend` *by* `divisor`.

**Note:** Assume we are dealing with an environment that could only store integers within the **32-bit** signed integer range: <code>[−2<sup>31</sup>, 2<sup>31</sup> − 1]</code>. For this problem, if the quotient is **strictly greater than** <code>2<sup>31</sup> - 1</code>, then return <code>2<sup>31</sup> - 1</code>, and if the quotient is **strictly less than** <code>-2<sup>31</sup></code>, then return <code>-2<sup>31</sup></code>.

#### Example 1:
<pre>
<strong>Input:</strong> dividend = 10, divisor = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> 10/3 = 3.33333.. which is truncated to 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> dividend = 7, divisor = -3
<strong>Output:</strong> -2
<strong>Explanation:</strong> 7/-3 = -2.33333.. which is truncated to -2.
</pre>

#### Constraints:
* <code>-2<sup>31</sup> <= dividend, divisor <= 2<sup>31</sup> - 1</code>
* `divisor != 0`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let is_neg = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0);
        let mut dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        let mut ret = 0;

        while dividend >= divisor {
            for i in 1..=32 {
                if divisor << i > dividend {
                    dividend -= divisor << (i - 1);
                    ret += 1 << (i - 1);
                    break;
                }
            }
        }

        if is_neg {
            ret = -ret;
        }

        ret
    }
}
```
