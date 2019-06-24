# 263. Ugly Number
Write a program to check whether a given number is an ugly number.

Ugly numbers are **positive numbers** whose prime factors only include <code>2, 3, 5</code>.

#### Example 1:
<pre>
<strong>Input:</strong> 6
<strong>Output:</strong> true
<strong>Explanation:</strong> 6 = 2 × 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 8
<strong>Output:</strong> true
<strong>Explanation:</strong> 8 = 2 × 2 × 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 14
<strong>Output:</strong> false
<strong>Explanation:</strong> 14 is not ugly since it includes another prime factor 7.
</pre>

#### Note:
1. <code>1</code> is typically treated as an ugly number.
2. Input is within the 32-bit signed integer range: [−2<sup>31</sup>,  2<sup>31</sup> − 1].

## Solutions

### 1. Solution (Rust)
```Rust
impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num == 0 {
            false
        } else if num == 1 {
            true
        } else if num % 2 == 0 {
            Self::is_ugly(num / 2)
        } else if num % 3 == 0 {
            Self::is_ugly(num / 3)
        } else if num % 5 == 0 {
            Self::is_ugly(num / 5)
        } else {
            false
        }
    }
}
```
