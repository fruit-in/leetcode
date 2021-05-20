# 1796. Second Largest Digit in a String
Given an alphanumeric string `s`, return *the **second largest** numerical digit that appears in* `s`, *or* `-1` *if it does not exist*.

An **alphanumeric** string is a string consisting of lowercase English letters and digits.

#### Example 1:
<pre>
<strong>Input:</strong> s = "dfa12321afd"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The digits that appear in s are [1, 2, 3]. The second largest digit is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abc1111"
<strong>Output:</strong> -1
<strong>Explanation:</strong> The digits that appear in s are [1]. There is no second largest digit.
</pre>

#### Constraints:
* `1 <= s.length <= 500`
* `s` consists of only lowercase English letters and/or digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut digits = [false; 10];

        for c in s.bytes() {
            if c.is_ascii_digit() {
                digits[(c - b'0') as usize] = true;
            }
        }

        (0..=9)
            .rev()
            .skip_while(|&x| !digits[x as usize])
            .skip(1)
            .find(|&x| digits[x as usize])
            .unwrap_or(-1)
    }
}
```
