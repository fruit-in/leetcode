# 10. Regular Expression Matching
Given an input string `s` and a pattern `p`, implement regular expression matching with support for `'.'` and `'*'` where:
* `'.'` Matches any single character.
* `'*'` Matches zero or more of the preceding element.

The matching should cover the **entire** input string (not partial).

#### Example 1:
<pre>
<strong>Input:</strong> s = "aa", p = "a"
<strong>Output:</strong> false
<strong>Explanation:</strong> "a" does not match the entire string "aa".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aa", p = "a*"
<strong>Output:</strong> true
<strong>Explanation:</strong> '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "ab", p = ".*"
<strong>Output:</strong> true
<strong>Explanation:</strong> ".*" means "zero or more (*) of any character (.)".
</pre>

#### Constraints:
* `1 <= s.length <= 20`
* `1 <= p.length <= 20`
* `s` contains only lowercase English letters.
* `p` contains only lowercase English letters, `'.'`, and `'*'`.
* It is guaranteed for each appearance of the character `'*'`, there will be a previous valid character to match.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let mut stack = vec![];
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for c in p.bytes() {
            if c != b'*' {
                stack.push(c);
            } else {
                match stack.pop() {
                    Some(b'.') => stack.push(b','),
                    Some(c) if c >= b'a' => stack.push(c - b'a' + b'A'),
                    _ => (),
                }
            }
        }

        for &c in &stack {
            let mut tmp = vec![false; s.len() + 1];

            if c == b',' || c.is_ascii_uppercase() {
                tmp[0] = dp[0];
            }

            for i in 1..=s.len() {
                match c {
                    b'.' => tmp[i] = dp[i - 1],
                    b'a'..=b'z' => tmp[i] = c == s[i - 1] && dp[i - 1],
                    _ => {
                        tmp[i] = dp[i];
                        for j in (0..i).rev() {
                            if !tmp[i] && (c == b',' || c - b'A' + b'a' == s[j]) {
                                tmp[i] = dp[j];
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            dp = tmp;
        }

        dp[s.len()]
    }
}
```
