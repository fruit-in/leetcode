# 76. Minimum Window Substring
Given two strings `s` and `t` of lengths `m` a`nd` n respectively, return *the **minimum window substring** of* `s` *such that every character in* `t` *(**including duplicates**) is included in the window*. If there is no such substring, return *the empty string* `""`.

The testcases will be generated such that the answer is **unique**.

#### Example 1:
<pre>
<strong>Input:</strong> s = "ADOBECODEBANC", t = "ABC"
<strong>Output:</strong> "BANC"
<strong>Explanation:</strong> The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "a", t = "a"
<strong>Output:</strong> "a"
<strong>Explanation:</strong> The entire string s is the minimum window.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "a", t = "aa"
<strong>Output:</strong> ""
<strong>Explanation:</strong> Both 'a's from t must be included in the window.
Since the largest window of s only has one 'a', return empty string.
</pre>

#### Constraints:
* `m == s.length`
* `n == t.length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* `s` and `t` consist of uppercase and lowercase English letters.

**Follow up:** Could you find an algorithm that runs in `O(m + n)` time?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }

        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut count_s = [0; 52];
        let mut count_t = [0; 52];
        let mut sub_indices = (0, s.len() + 1);
        let mut i = 0;
        let mut j = 0;

        while j < t.len() {
            match s[j] {
                b'A'..=b'Z' => count_s[(s[j] - b'A') as usize] += 1,
                _ => count_s[(s[j] - b'a') as usize + 26] += 1,
            }
            match t[j] {
                b'A'..=b'Z' => count_t[(t[j] - b'A') as usize] += 1,
                _ => count_t[(t[j] - b'a') as usize + 26] += 1,
            }

            j += 1;
        }

        if (0..52).all(|k| count_s[k] >= count_t[k]) {
            sub_indices = (i, j);
        }

        while j < s.len() {
            while j < s.len() && (0..52).any(|k| count_s[k] < count_t[k]) {
                match s[j] {
                    b'A'..=b'Z' => count_s[(s[j] - b'A') as usize] += 1,
                    _ => count_s[(s[j] - b'a') as usize + 26] += 1,
                }

                j += 1;
            }

            while (0..52).all(|k| count_s[k] >= count_t[k]) {
                if j - i < sub_indices.1 - sub_indices.0 {
                    sub_indices = (i, j);
                }

                match s[i] {
                    b'A'..=b'Z' => count_s[(s[i] - b'A') as usize] -= 1,
                    _ => count_s[(s[i] - b'a') as usize + 26] -= 1,
                }

                i += 1;
            }
        }

        if sub_indices.1 - sub_indices.0 > s.len() {
            return String::new();
        }

        String::from_utf8(s[sub_indices.0..sub_indices.1].to_vec()).unwrap()
    }
}
```
