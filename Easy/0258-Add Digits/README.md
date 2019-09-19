# 258. Add Digits
Given a non-negative integer ```num```, repeatedly add all its digits until the result has only one digit.

#### Example:
<pre>
<strong>Input:</strong> 38
<strong>Output:</strong> 2
<strong>Explanation:</strong> The process is like: 3 + 8 = 11, 1 + 1 = 2. 
             Since 2 has only one digit, return it.
</pre>

#### Follow up:
Could you do it without any loop/recursion in O(1) runtime?

## Solutions (Rust)

### 1. Iteration
```Rust
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        while num >= 10 {
            let mut tmp = 0;
            while num != 0 {
                tmp += num % 10;
                num /= 10;
            }
            num = tmp;
        }
        num
    }
}
```

### 2. Mod Operator
```Rust
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        (num - 1) % 9 + 1
    }
}
```
