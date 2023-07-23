# 451. Sort Characters By Frequency
Given a string, sort it in decreasing order based on the frequency of characters.

#### Example 1:
<pre>
<b>Input:</b> "tree"
<b>Output:</b> "eert"
<b>Explanation:</b> 'e' appears twice while 'r' and 't' both appear once.
So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.
</pre>

#### Example 2:
<pre>
<b>Input:</b> "cccaaa"
<b>Output:</b> "cccaaa"
<b>Explanation:</b> Both 'c' and 'a' appear three times, so "aaaccc" is also a valid answer.
Note that "cacaca" is incorrect, as the same characters must be together.
</pre>

#### Example 3:
<pre>
<b>Input:</b> "Aabb"
<b>Output:</b> "bbAa"
<b>Explanation:</b> "bbaA" is also a valid answer, but "Aabb" is incorrect.
Note that 'A' and 'a' are treated as two different characters.
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut counter = HashMap::new();

        for ch in s.clone() {
            *counter.entry(ch).or_insert(0) -= 1;
        }

        s.sort_unstable();
        s.sort_by_key(|k| counter.get(&k));

        s.iter().collect()
    }
}
```
