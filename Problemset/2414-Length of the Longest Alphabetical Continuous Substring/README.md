# 2414. Length of the Longest Alphabetical Continuous Substring
An **alphabetical continuous string** is a string consisting of consecutive letters in the alphabet. In other words, it is any substring of the string `"abcdefghijklmnopqrstuvwxyz"`.

* For example, `"abc"` is an alphabetical continuous string, while `"acb"` and `"za"` are not.

Given a string `s` consisting of lowercase letters only, return the *length of the **longest** alphabetical continuous substring*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abacaba"
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 4 distinct continuous substrings: "a", "b", "c" and "ab".
"ab" is the longest continuous substring.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcde"
<strong>Output:</strong> 5
<strong>Explanation:</strong> "abcde" is the longest continuous substring.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of only English lowercase letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 1;
        let mut ret = 1;

        for i in 1..s.len() {
            if s[i - 1] + 1 == s[i] {
                count += 1;
                ret = ret.max(count);
            } else {
                count = 1;
            }
        }

        ret
    }
}
```
