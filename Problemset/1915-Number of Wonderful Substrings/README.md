# 1915. Number of Wonderful Substrings
A **wonderful** string is a string where **at most one** letter appears an **odd** number of times.

* For example, `"ccjjc"` and `"abab"` are wonderful, but `"ab"` is not.

Given a string `word` that consists of the first ten lowercase English letters (`'a'` through `'j'`), return *the **number of wonderful non-empty substrings*** in `word`. *If the same substring appears multiple times in* `word`, *then count **each occurrence** separately*.

A **substring** is a contiguous sequence of characters in a string.

#### Example 1:
<pre>
<strong>Input:</strong> word = "aba"
<strong>Output:</strong> 4
<strong>Explanation:</strong> The four wonderful substrings are underlined below:
- "aba" -> "a"
- "aba" -> "b"
- "aba" -> "a"
- "aba" -> "aba"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word = "aabb"
<strong>Output:</strong> 9
<strong>Explanation:</strong> The nine wonderful substrings are underlined below:
- "aabb" -> "a"
- "aabb" -> "aa"
- "aabb" -> "aab"
- "aabb" -> "aabb"
- "aabb" -> "a"
- "aabb" -> "abb"
- "aabb" -> "b"
- "aabb" -> "bb"
- "aabb" -> "b"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> word = "he"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The two wonderful substrings are underlined below:
- "he" -> "h"
- "he" -> "e"
</pre>

#### Constraints:
* <code>1 <= word.length <= 10<sup>5</sup></code>
* `word` consists of lowercase English letters from `'a'` to `'j'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let word = word.as_bytes();
        let mut count = [0; 1024];
        let mut mask = 0;
        let mut ret = 0;
        count[0] = 1;

        for i in 0..word.len() {
            mask ^= 1 << (word[i] - b'a');
            for j in 0..10 {
                ret += count[mask ^ (1 << j)];
            }
            ret += count[mask];
            count[mask] += 1;
        }

        ret
    }
}
```
