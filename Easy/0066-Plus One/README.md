# 66. Plus One
Given a **non-empty** array of digits representing a non-negative integer, plus one to the integer.

The digits are stored such that the most significant digit is at the head of the list, and each element in the array contain a single digit.

You may assume the integer does not contain any leading zero, except the number 0 itself.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3]
<strong>Output:</strong> [1,2,4]
<strong>Explanation:</strong> The array represents the integer 123.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [4,3,2,1]
<strong>Output:</strong> [4,3,2,2]
<strong>Explanation:</strong> The array represents the integer 4321.
</pre>

## Solutions

### 1. Solution (Rust)
```Rust
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len() - 1;
        loop {
            if digits[i] != 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
                if i > 0 {
                    i -= 1;
                } else {
                    digits.insert(0, 1);
                    return digits;
                }
            }
        }
    }
}
```
