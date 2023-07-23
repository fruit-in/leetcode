# 2427. Number of Common Factors
Given two positive integers `a` and `b`, return *the number of **common** factors of* `a` *and* `b`.

An integer `x` is `a` **common factor** of `a` and `b` if `x` divides both `a` and `b`.

#### Example 1:
<pre>
<strong>Input:</strong> a = 12, b = 6
<strong>Output:</strong> 4
<strong>Explanation:</strong> The common factors of 12 and 6 are 1, 2, 3, 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = 25, b = 30
<strong>Output:</strong> 2
<strong>Explanation:</strong> The common factors of 25 and 30 are 1, 5.
</pre>

#### Constraints:
* `1 <= a, b <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        (1..=a.min(b)).filter(|x| a % x == 0 && b % x == 0).count() as i32
    }
}
```
