# 2165. Smallest Value of the Rearranged Number
You are given an integer `num`. **Rearrange** the digits of `num` such that its value is **minimized** and it does not contain **any** leading zeros.

Return *the rearranged number with minimal value*.

Note that the sign of the number does not change after rearranging the digits.

#### Example 1:
<pre>
<strong>Input:</strong> num = 310
<strong>Output:</strong> 103
<strong>Explanation:</strong> The possible arrangements for the digits of 310 are 013, 031, 103, 130, 301, 310.
The arrangement with the smallest value that does not contain any leading zeros is 103.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = -7605
<strong>Output:</strong> -7650
<strong>Explanation:</strong> Some possible arrangements for the digits of -7605 are -7650, -6705, -5076, -0567.
The arrangement with the smallest value that does not contain any leading zeros is -7650.
</pre>

#### Constraints:
* <code>-10<sup>15</sup> <= num <= 10<sup>15</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let mut digits = num.abs().to_string().into_bytes();

        digits.sort_unstable();
        if num >= 0 {
            for i in 0..digits.len() {
                if digits[i] != b'0' {
                    digits.swap(0, i);
                    break;
                }
            }
        } else {
            digits.push(b'-');
            digits.reverse();
        }

        String::from_utf8(digits).unwrap().parse().unwrap()
    }
}
```
