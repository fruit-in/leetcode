# 1316. Distinct Echo Substrings
Return the number of distinct non-empty substrings of text that can be written as the concatenation of some string with itself (i.e. it can be written as a + a where a is some string).

#### Example 1:
<pre>
<strong>Input:</strong> text = "abcabcabc"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The 3 substrings are "abcabc", "bcabca" and "cabcab".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> text = "leetcodeleetcode"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The 2 substrings are "ee" and "leetcodeleetcode".
</pre>

#### Constraints:
* `1 <= text.length <= 2000`
* `text` has only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let bytes = text.as_bytes();
        let mut subends: HashMap<&str, HashSet<usize>> = HashMap::new();
        let mut halfsubs = HashSet::new();

        for i in 0..text.len() {
            for j in i..=((text.len() + i - 2) / 2)
                .max((2 * i).saturating_sub(1))
                .min(text.len() - 1)
            {
                if i > 0
                    && j < text.len() - 1
                    && bytes[i - 1] != bytes[j]
                    && bytes[i] != bytes[j + 1]
                {
                    continue;
                }

                let s = text.get(i..=j).unwrap();

                if halfsubs.contains(&s) {
                    continue;
                }

                if i > 0
                    && j <= 2 * i - 1
                    && subends
                        .get(&s)
                        .unwrap_or(&HashSet::new())
                        .contains(&(i - 1))
                {
                    halfsubs.insert(s);
                } else if 2 * j < text.len() - 1 + i {
                    subends.entry(s).or_insert(HashSet::new()).insert(j);
                }
            }
        }

        halfsubs.len() as i32
    }
}
```
