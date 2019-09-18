# 172. Factorial Trailing Zeroes
Given an integer *n*, return the number of trailing zeroes in *n*!.

#### Example 1:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> 3! = 6, no trailing zero.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> 1
<strong>Explanation:</strong> 5! = 120, one trailing zero.
</pre>

**Note:** Your solution should be in logarithmic time complexity.

## Solutions (Rust)

### 1. Count 5
```Rust
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut zeroes = 0;
        while n >= 5 {
            n /= 5;
            zeroes += n;
        }
        zeroes
    }
}
```
