# 1297. Maximum Number of Occurrences of a Substring
Given a string `s`, return the maximum number of occurrences of **any** substring under the following rules:

* The number of unique characters in the substring must be less than or equal to `maxLetters`.
* The substring size must be between `minSize` and `maxSize` inclusive.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
<strong>Output:</strong> 2
<strong>Explanation:</strong> Substring "aab" has 2 occurrences in the original string.
It satisfies the conditions, 2 unique letters and size 3 (between minSize and maxSize).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> Substring "aaa" occur 2 times in the string. It can overlap.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `1 <= maxLetters <= 26`
* `1 <= minSize <= maxSize <= min(26, s.length)`
* `s` consists of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let s = s.as_bytes();
        let min_size = min_size as usize;
        let mut letter_count = [0; 26];
        let mut unique_count = 0;
        let mut substring_count = HashMap::new();

        for i in 0..min_size {
            letter_count[(s[i] - b'a') as usize] += 1;
            unique_count += (letter_count[(s[i] - b'a') as usize] == 1) as i32;
        }
        if unique_count <= max_letters {
            substring_count.insert(&s[..min_size], 1);
        }

        for i in min_size..s.len() {
            letter_count[(s[i - min_size] - b'a') as usize] -= 1;
            unique_count -= (letter_count[(s[i - min_size] - b'a') as usize] == 0) as i32;
            letter_count[(s[i] - b'a') as usize] += 1;
            unique_count += (letter_count[(s[i] - b'a') as usize] == 1) as i32;
            if unique_count <= max_letters {
                *substring_count.entry(&s[i - min_size + 1..=i]).or_insert(0) += 1;
            }
        }

        *substring_count.values().max().unwrap_or(&0)
    }
}
```
