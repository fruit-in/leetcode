# 1392. Longest Happy Prefix
A string is called a **happy prefix** if is a **non-empty** prefix which is also a suffix (excluding itself).

Given a string `s`, return *the **longest happy prefix** of* `s`. Return an empty string `""` if no such prefix exists.

#### Example 1:
<pre>
<strong>Input:</strong> s = "level"
<strong>Output:</strong> "l"
<strong>Explanation:</strong> s contains 4 prefix excluding itself ("l", "le", "lev", "leve"), and suffix ("l", "el", "vel", "evel"). The largest prefix which is also suffix is given by "l".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "ababab"
<strong>Output:</strong> "abab"
<strong>Explanation:</strong> "abab" is the largest prefix which is also suffix. They can overlap in the original string.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` contains only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let s = s.as_bytes();
        let mut j = 0;
        let mut next = vec![0; s.len()];

        for i in 1..s.len() {
            while j > 0 && s[i] != s[j] {
                j = next[j - 1];
            }

            if s[i] == s[j] {
                j += 1;
                next[i] = j;
            }
        }

        String::from_utf8(s[..next[s.len() - 1]].to_vec()).unwrap()
    }
}
```
