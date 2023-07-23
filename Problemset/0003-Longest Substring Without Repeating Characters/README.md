# 3. Longest Substring Without Repeating Characters
Given a string, find the length of the **longest substring** without repeating characters.

#### Example 1:
<pre>
<strong>Input:</strong> "abcabcbb"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The answer is "abc", with the length of 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "bbbbb"
<strong>Output:</strong> 1
<strong>Explanation:</strong> The answer is "b", with the length of 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "pwwkew"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The answer is "wke", with the length of 3.
             Note that the answer must be a <strong>substring</strong>, "pwke" is a <i>subsequence</i> and not a substring.
</pre>

## Solutions (Rust)

### 1. Sliding Window
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut indices = HashMap::new();
        let mut ret = 0;
        let mut i = 0;

        for j in 0..s.len() {
            match indices.insert(s[j], j) {
                Some(x) => i = i.max(x + 1),
                None => (),
            }
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
```
