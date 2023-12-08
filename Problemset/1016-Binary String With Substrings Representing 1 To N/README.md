# 1016. Binary String With Substrings Representing 1 To N
Given a binary string `s` and a positive integer `n`, return `true` *if the binary representation of all the integers in the range* `[1, n]` *are **substrings** of* `s`, *or* `false` *otherwise*.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "0110", n = 3
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "0110", n = 4
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s[i]` is either `'0'` or `'1'`.
* <code>1 <= n <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for x in 1..=n {
            if !s.contains(&format!("{:b}", x)) {
                return false;
            }
        }

        true
    }
}
```
