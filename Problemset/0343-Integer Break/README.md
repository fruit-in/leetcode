# 343. Integer Break
Given a positive integer *n*, break it into the sum of **at least** two positive integers and maximize the product of those integers. Return the maximum product you can get.

#### Example 1:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> 2 = 1 + 1, 1 × 1 = 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 10
<strong>Output:</strong> 36
<strong>Explanation:</strong> 10 = 3 + 3 + 4, 3 × 3 × 4 = 36.
</pre>

**Note:** You may assume that *n* is not less than 2 and not larger than 58.

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match (n, (n - 2) / 3) {
            (2, _) | (3, _) => n - 1,
            (_, x) => 3i32.pow(x as u32) * (n - 3 * x),
        }
    }
}
```
