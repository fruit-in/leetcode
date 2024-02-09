# 2609. Find the Longest Balanced Substring of a Binary String
You are given a binary string `s` consisting only of zeroes and ones.

A substring of `s` is considered balanced if **all zeroes are before ones** and the number of zeroes is equal to the number of ones inside the substring. Notice that the empty substring is considered a balanced substring.

Return *the length of the longest balanced substring of* `s`.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "01000111"
<strong>Output:</strong> 6
<strong>Explanation:</strong> The longest balanced substring is "000111", which has length 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "00111"
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest balanced substring is "0011", which has length 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "111"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no balanced substring except the empty substring, so the answer is 0.
</pre>

#### Constraints:
* `1 <= s.length <= 50`
* `'0' <= s[i] <= '1'`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count0 = (b'1' - s[0]) as i32;
        let mut count1 = 0;
        let mut ret = 0;

        for i in 1..s.len() {
            if s[i] == b'0' && s[i - 1] == b'1' {
                count0 = 0;
                count1 = 0;
            }

            if s[i] == b'0' {
                count0 += 1;
            } else if s[i] == b'1' {
                count1 += 1;
            }

            ret = ret.max(count0.min(count1) * 2);
        }

        ret
    }
}
```
