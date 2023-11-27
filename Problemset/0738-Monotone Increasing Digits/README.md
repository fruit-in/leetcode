# 738. Monotone Increasing Digits
An integer has **monotone increasing digits** if and only if each pair of adjacent digits `x` and `y` satisfy `x <= y`.

Given an integer `n`, return *the largest number that is less than or equal to* `n` *with **monotone increasing digits***.

#### Example 1:
<pre>
<strong>Input:</strong> n = 10
<strong>Output:</strong> 9
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 1234
<strong>Output:</strong> 1234
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 332
<strong>Output:</strong> 299
</pre>

#### Constraints:
* <code>0 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits = n.to_string().into_bytes();
        let mut i = digits.len();

        while let Some(j) = (0..digits.len() - 1).find(|&j| digits[j] > digits[j + 1]) {
            digits[j] -= 1;
            for k in j + 1..i {
                digits[k] = b'9';
            }
            i = j + 1;
        }

        String::from_utf8(digits).unwrap().parse().unwrap()
    }
}
```
