# 1012. Numbers With Repeated Digits
Given an integer `n`, return *the number of positive integers in the range* `[1, n]` *that have **at least one** repeated digit*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 20
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only positive number (<= 20) with at least 1 repeated digit is 11.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 100
<strong>Output:</strong> 10
<strong>Explanation:</strong> The positive numbers (<= 100) with atleast 1 repeated digit are 11, 22, 33, 44, 55, 66, 77, 88, 99, and 100.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 1000
<strong>Output:</strong> 262
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let digits = n
            .to_string()
            .bytes()
            .map(|ch| (ch - b'0') as i32)
            .collect::<Vec<_>>();
        let mut ret = n;

        for i in 0..digits.len() - 1 {
            let mut x = 9;

            for j in (10 - i as i32)..=9 {
                x *= j;
            }

            ret -= x;
        }

        for i in 0..digits.len() {
            let mut count = (i == 0) as i32;

            for j in 0..i {
                if digits[j] < digits[i] {
                    count += 1;
                }
            }

            let mut x = digits[i] - count;

            for j in (11 - digits.len() as i32)..(10 - i as i32) {
                x *= j;
            }

            ret -= x;

            if digits[..i].contains(&digits[i]) {
                break;
            }

            if i == digits.len() - 1 {
                ret -= 1;
            }
        }

        ret
    }
}
```
