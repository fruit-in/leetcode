# 409. Longest Palindrome
Given a string which consists of lowercase or uppercase letters, find the length of the longest palindromes that can be built with those letters.

This is case sensitive, for example <code>"Aa"</code> is not considered a palindrome here.

#### Note:
Assume the length of given string will not exceed 1,010. 

#### Example:
<pre>
<strong>Input:</strong> "abccccdd"
<strong>Output:</strong> 7
<strong>Explanation:</strong>
One longest palindrome that can be built is "dccaccd", whose length is 7.
</pre>

## Solutions

### 1. Greedy (Rust)
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = HashMap::new();
        let mut result = 0;
        for ch in s.chars() {
            match count.get(&ch) {
                Some(i) => count.insert(ch, i + 1),
                None => count.insert(ch, 1),
            };
        }
        for v in count.values() {
            if v % 2 == 0 || result % 2 == 0 {
                result += v;
            } else {
                result += v - 1;
            }
        }
        result
    }
}
```
