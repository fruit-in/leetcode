# 1015. Smallest Integer Divisible by K
Given a positive integer `k`, you need to find the **length** of the **smallest** positive integer `n` such that `n` is divisible by `k`, and `n` only contains the digit `1`.

Return *the **length** of* `n`. If there is no such `n`, return -1.

**Note:** `n` may not fit in a 64-bit signed integer.

#### Example 1:
<pre>
<strong>Input:</strong> k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The smallest answer is n = 1, which has length 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> k = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no such positive integer n divisible by 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> k = 3
<strong>Output:</strong> 3
<strong>Explanation:</strong> The smallest answer is n = 111, which has length 3.
</pre>

#### Constraints:
* <code>1 <= k <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut rem = 0;

        for n in 1..=k {
            rem = (rem * 10 % k + 1 % k) % k;

            if rem == 0 {
                return n;
            }
        }

        -1
    }
}
```
