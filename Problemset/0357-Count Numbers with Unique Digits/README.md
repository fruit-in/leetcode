# 357. Count Numbers with Unique Digits
Given a **non-negative** integer n, count all numbers with unique digits, x, where 0 ≤ x < 10<sup>n</sup>.

#### Example:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> 91
<strong>Explanation:</strong> The answer should be the total numbers in the range of 0 ≤ x < 100, 
             excluding 11,22,33,44,55,66,77,88,99
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
        let mut n = n.min(10) as usize;
        let mut ret = 1;

        while n > 0 {
            ret += 9 * factorials[9] / factorials[10 - n];
            n -= 1;
        }

        ret
    }
}
```
