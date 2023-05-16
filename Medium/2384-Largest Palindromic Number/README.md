# 2384. Largest Palindromic Number
You are given a string `num` consisting of digits only.

Return *the **largest palindromic** integer (in the form of a string) that can be formed using digits taken from* `num`. It should not contain **leading zeroes**.

**Notes:**

* You do **not** need to use all the digits of `num`, but you must use **at least** one digit.
* The digits can be reordered.

#### Example 1:
<pre>
<strong>Input:</strong> num = "444947137"
<strong>Output:</strong> "7449447"
<strong>Explanation:</strong>
Use the digits "4449477" from "444947137" to form the palindromic integer "7449447".
It can be shown that "7449447" is the largest palindromic integer that can be formed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = "00009"
<strong>Output:</strong> "9"
<strong>Explanation:</strong>
It can be shown that "9" is the largest palindromic integer that can be formed.
Note that the integer returned should not contain leading zeroes.
</pre>

#### Constraints:
* <code>1 <= num.length <= 10<sup>5</sup></code>
* `num` consists of digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut count = [0; 10];
        let mut digits = vec![];

        for digit in num.bytes() {
            count[(digit - b'0') as usize] += 1;
        }

        for i in (0..=9).rev() {
            while count[i] > 1 {
                count[i] -= 2;
                digits.push(i as u8 + b'0');
            }
        }

        if *digits.get(0).unwrap_or(&b'1') == b'0' {
            digits.clear();
        }

        let mut digits_rev = digits.clone();
        digits_rev.reverse();

        if let Some(i) = (0..=9).rev().find(|&i| count[i] > 0) {
            digits.push(i as u8 + b'0');
        }
        digits.append(&mut digits_rev);
        if digits.is_empty() {
            digits.push(b'0');
        }

        String::from_utf8(digits).unwrap()
    }
}
```
