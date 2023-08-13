# 2506. Count Pairs Of Similar Strings
You are given a **0-indexed** string array `words`.

Two strings are **similar** if they consist of the same characters.

* For example, `"abca"` and `"cba"` are similar since both consist of characters `'a'`, `'b'`, and `'c'`.
* However, `"abacba"` and `"bcfd"` are not similar since they do not consist of the same characters.

Return *the number of pairs* `(i, j)` *such that* `0 <= i < j <= word.length - 1` *and the two strings* `words[i]` *and* `words[j]` *are similar*.

#### Example 1:
<pre>
<strong>Input:</strong> words = ["aba","aabb","abcd","bac","aabc"]
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 pairs that satisfy the conditions:
- i = 0 and j = 1 : both words[0] and words[1] only consist of characters 'a' and 'b'.
- i = 3 and j = 4 : both words[3] and words[4] only consist of characters 'a', 'b', and 'c'.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> words = ["aabb","ab","ba"]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 pairs that satisfy the conditions:
- i = 0 and j = 1 : both words[0] and words[1] only consist of characters 'a' and 'b'.
- i = 0 and j = 2 : both words[0] and words[2] only consist of characters 'a' and 'b'.
- i = 1 and j = 2 : both words[1] and words[2] only consist of characters 'a' and 'b'.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> words = ["nba","cba","dba"]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Since there does not exist any pair that satisfies the conditions, we return 0.
</pre>

#### Constraints:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` consist of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count = HashMap::new();

        for word in &words {
            let x = word.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')));

            count.entry(x).and_modify(|c| *c += 1).or_insert(1);
        }

        count.values().map(|c| c * (c - 1) / 2).sum()
    }
}
```
