# 231. Power of Two
Given an integer, write a function to determine if it is a power of two.

#### Example 1:
<pre>
<strong>Input:</strong> 1
<strong>Output:</strong> true
<strong>Explanation:</strong> 2<sup>0</sup> = 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 16
<strong>Output:</strong> true
<strong>Explanation:</strong> 2<sup>4</sup> = 16
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 218
<strong>Output:</strong> false
</pre>

## Solutions (Rust)

### 1. n & (n - 1)
```Rust
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            false
        } else {
            n & (n - 1) == 0
        }
    }
}
```

### 2. n / 2
```Rust
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            false
        } else if n == 1 {
            true
        } else {
            n % 2 == 0 && Self::is_power_of_two(n / 2)
        }
    }
}
```
