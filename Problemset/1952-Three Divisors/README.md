# 1952. Three Divisors
Given an integer `n`, return `true` *if* `n` *has **exactly three positive divisors**. Otherwise, return* `false`.

An integer `m` is a **divisor** of `n` if there exists an integer `k` such that `n = k * m`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> false
<strong>Explanation:</strong> 2 has only two divisors: 1 and 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> 4 has three divisors: 1, 2, and 4.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_three(n: i32) -> bool {
        (1..=n).filter(|m| n % m == 0).count() == 3
    }
}
```
