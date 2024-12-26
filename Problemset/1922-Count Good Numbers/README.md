# 1922. Count Good Numbers
A digit string is **good** if the digits **(0-indexed)** at **even** indices are **even** and the digits at **odd** indices are **prime** (`2`, `3`, `5`, or `7`).

* For example, `"2582"` is good because the digits (`2` and `8`) at even positions are even and the digits (`5` and `2`) at odd positions are prime. However, `"3245"` is not good because `3` is at an even index but is **not** even.

Given an integer `n`, return *the **total** number of good digit strings of length* `n`. Since the answer may be large, **return it modulo** <code>10<sup>9</sup> + 7</code>.

A **digit string** is a string consisting of digits `0` through `9` that may contain leading zeros.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 5
<strong>Explanation:</strong> The good numbers of length 1 are "0", "2", "4", "6", "8".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> 400
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 50
<strong>Output:</strong> 564908303
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>15</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        fn twenty_pow(n: i64) -> i64 {
            if n == 0 {
                1
            } else if n % 2 == 0 {
                twenty_pow(n / 2) * twenty_pow(n / 2) % 1_000_000_007
            } else {
                twenty_pow(n - 1) * 20 % 1_000_000_007
            }
        }

        (match n % 2 {
            0 => twenty_pow(n / 2),
            _ => twenty_pow((n - 1) / 2) * 5 % 1_000_000_007,
        }) as i32
    }
}
```
