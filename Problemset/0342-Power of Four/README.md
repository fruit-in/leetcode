# 342. Power of Four
Given an integer (signed 32 bits), write a function to check whether it is a power of 4.

#### Example 1:
<pre>
<strong>Input:</strong> 16
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> false
</pre>

**Follow up:** Could you solve it without loops/recursion?

## Solutions (Rust)

### 1. num div 4
```Rust
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        let mut num = num;
        if num <= 0 {
            false
        } else {
            while num % 4 == 0 {
                num /= 4;
            }
            num == 1
        }
    }
}
```

### 2. Bitwise Operator
```Rust
impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        (num > 0) && (num & 0x55555555 == num) && (num & (num - 1) == 0)
    }
}
```
