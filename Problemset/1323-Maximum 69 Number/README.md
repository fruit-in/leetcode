# 1323. Maximum 69 Number
Given a positive integer ```num``` consisting only of digits 6 and 9.

Return the maximum number you can get by changing **at most** one digit (6 becomes 9, and 9 becomes 6).

#### Example 1:
<pre>
<strong>Input:</strong> num = 9669
<strong>Output:</strong> 9969
<strong>Explanation:</strong>
Changing the first digit results in 6669.
Changing the second digit results in 9969.
Changing the third digit results in 9699.
Changing the fourth digit results in 9666.
The maximum number is 9969.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 9996
<strong>Output:</strong> 9999
<strong>Explanation:</strong> Changing the last digit 6 to 9 results in the maximum number.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = 9999
<strong>Output:</strong> 9999
<strong>Explanation:</strong> It is better not to apply any change.
</pre>

#### Constraints:
* ```1 <= num <= 10^4```
* ```num```'s digits are 6 or 9.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut i = 0;

        for i in (0..5).rev() {
            if num / 10_i32.pow(i) % 10 == 6 {
                return num + 3 * 10_i32.pow(i);
            }
        }

        num
    }
}
```
