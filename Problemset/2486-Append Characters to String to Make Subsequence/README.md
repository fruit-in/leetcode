# 2486. Append Characters to String to Make Subsequence
You are given two strings `s` and `t` consisting of only lowercase English letters.

Return *the minimum number of characters that need to be appended to the end of* `s` *so that* `t` *becomes a **subsequence** of* `s`.

A **subsequence** is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

#### Example 1:
<pre>
<strong>Input:</strong> s = "coaching", t = "coding"
<strong>Output:</strong> 4
<strong>Explanation:</strong> Append the characters "ding" to the end of s so that s = "coachingding".
Now, t is a subsequence of s ("coachingding").
It can be shown that appending any 3 characters to the end of s will never make t a subsequence.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcde", t = "a"
<strong>Output:</strong> 0
<strong>Explanation:</strong> t is already a subsequence of s ("abcde").
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "z", t = "abcde"
<strong>Output:</strong> 5
<strong>Explanation:</strong> Append the characters "abcde" to the end of s so that s = "zabcde".
Now, t is a subsequence of s ("zabcde").
It can be shown that appending any 4 characters to the end of s will never make t a subsequence.
</pre>

#### Constraints:
* <code>1 <= s.length, t.length <= 10<sup>5</sup></code>
* `s` and `t` consist only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;

        for j in 0..s.len() {
            if i >= t.len() {
                break;
            } else if t[i] == s[j] {
                i += 1;
            }
        }

        (t.len() - i) as i32
    }
}
```
