# 2544. Alternating Digit Sum
You are given a positive integer `n`. Each digit of `n` has a sign according to the following rules:

* The **most significant digit** is assigned a **positive** sign.
* Each other digit has an opposite sign to its adjacent digits.

Return *the sum of all digits with their corresponding sign*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 521
<strong>Output:</strong> 4
<strong>Explanation:</strong> (+5) + (-2) + (+1) = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 111
<strong>Output:</strong> 1
<strong>Explanation:</strong> (+1) + (-1) + (+1) = 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 886996
<strong>Output:</strong> 0
<strong>Explanation:</strong> (+8) + (-8) + (+6) + (-9) + (+9) + (-6) = 0.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut n = n;
        let mut ret = 0;

        while n > 0 {
            ret = n % 10 - ret;
            n /= 10;
        }

        ret
    }
}
```
