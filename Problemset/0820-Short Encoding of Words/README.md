# 820. Short Encoding of Words
A **valid encoding** of an array of `words` is any reference string `s` and array of indices `indices` such that:
* `words.length == indices.length`
* The reference string `s` ends with the `'#'` character.
* For each index `indices[i]`, the **substring** of `s` starting from `indices[i]` and up to (but not including) the next `'#'` character is equal to `words[i]`.

Given an array of `words`, return *the **length of the shortest reference string*** `s` *possible of any **valid encoding** of* `words`.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["time", "me", "bell"]
<strong>Output:</strong> 10
<strong>Explanation:</strong> A valid encoding would be s = "time#bell#" and indices = [0, 2, 5].
words[0] = "time", the substring of s starting from indices[0] = 0 to the next '#' is underlined in "time#bell#"
words[1] = "me", the substring of s starting from indices[1] = 2 to the next '#' is underlined in "time#bell#"
words[2] = "bell", the substring of s starting from indices[2] = 5 to the next '#' is underlined in "time#bell#"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["t"]
<strong>Output:</strong> 2
<strong>Explanation:</strong> A valid encoding would be s = "t#" and indices = [0].
</pre>

#### Constraints:
* `1 <= words.length <= 2000`
* `1 <= words[i].length <= 7`
* `words[i]` consists of only lowercase letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let words = words.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        let mut suffixes = HashSet::new();

        for w in &words {
            for i in 1..w.len() {
                suffixes.insert(w.get(i..).unwrap());
            }
        }

        words
            .difference(&suffixes)
            .map(|w| w.len() as i32 + 1)
            .sum()
    }
}
```
