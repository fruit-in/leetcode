# 2235. Add Two Integers
Given two integers `num1` and `num2`, return *the **sum** of the two integers*.

#### Example 1:
<pre>
<strong>Input:</strong> num1 = 12, num2 = 5
<strong>Output:</strong> 17
<strong>Explanation:</strong> num1 is 12, num2 is 5, and their sum is 12 + 5 = 17, so 17 is returned.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num1 = -10, num2 = 4
<strong>Output:</strong> -6
<strong>Explanation:</strong> num1 + num2 = -6, so -6 is returned.
</pre>

#### Constraints:
* `-100 <= num1, num2 <= 100`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def sum(self, num1: int, num2: int) -> int:
        return num1 + num2
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}
```
