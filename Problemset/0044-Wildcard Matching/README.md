# 44. Wildcard Matching
Given an input string (`s`) and a pattern (`p`), implement wildcard pattern matching with support for `'?'` and `'*'` where:

* `'?'` Matches any single character.
* `'*'` Matches any sequence of characters (including the empty sequence).

The matching should cover the **entire** input string (not partial).

#### Example 1:
<pre>
<strong>Input:</strong> s = "aa", p = "a"
<strong>Output:</strong> false
<strong>Explanation:</strong> "a" does not match the entire string "aa".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aa", p = "*"
<strong>Output:</strong> true
<strong>Explanation:</strong> '*' matches any sequence.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "cb", p = "?a"
<strong>Output:</strong> false
<strong>Explanation:</strong> '?' matches 'c', but the second letter is 'a', which does not match 'b'.
</pre>

#### Constraints:
* `0 <= s.length, p.length <= 2000`
* `s` contains only lowercase English letters.
* `p` contains only lowercase English letters, `'?'` or `'*'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..=p.len() {
            let mut tmp = vec![false; s.len() + 1];
            let mut prefix_or = dp[0];

            if p[i - 1] == b'*' {
                tmp[0] = dp[0];
            }
            for j in 1..=s.len() {
                prefix_or |= dp[j];
                match p[i - 1] {
                    b'?' => tmp[j] = dp[j - 1],
                    b'*' => tmp[j] = prefix_or,
                    _ => tmp[j] = p[i - 1] == s[j - 1] && dp[j - 1],
                }
            }

            dp = tmp;
        }

        dp[s.len()]
    }
}
```
