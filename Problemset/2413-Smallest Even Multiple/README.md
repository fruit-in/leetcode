# 2413. Smallest Even Multiple
Given a **positive** integer `n`, return *the smallest positive integer that is a multiple of **both*** `2` *and* `n`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 10
<strong>Explanation:</strong> The smallest multiple of both 5 and 2 is 10.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> The smallest multiple of both 6 and 2 is 6. Note that a number is a multiple of itself.
</pre>

#### Constraints:
* `1 <= n <= 150`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        match n % 2 {
            0 => n,
            _ => 2 * n,
        }
    }
}
```
