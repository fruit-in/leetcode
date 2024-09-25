# 522. Longest Uncommon Subsequence II
Given an array of strings `strs`, return *the length of the **longest uncommon subsequence** between them*. If the longest uncommon subsequence does not exist, return `-1`.

An **uncommon subsequence** between an array of strings is a string that is a **subsequence of one string but not the others**.

A **subsequence** of a string `s` is a string that can be obtained after deleting any number of characters from `s`.

* For example, `"abc"` is a subsequence of `"aebdc"` because you can delete the underlined characters in `"aebdc"` to get `"abc"`. Other subsequences of `"aebdc"` include `"aebdc"`, `"aeb"`, and `""` (empty string).

#### Example 1:
<pre>
<strong>Input:</strong> strs = ["aba","cdc","eae"]
<strong>Output:</strong> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> strs = ["aaa","aaa","aa"]
<strong>Output:</strong> -1
</pre>

#### Constraints:
* `2 <= strs.length <= 50`
* `1 <= strs[i].length <= 10`
* `strs[i]` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut count = HashMap::new();

        for s in &strs {
            let s = s.as_bytes();
            let mut subs = HashSet::new();

            for x in 1..2_i32.pow(s.len() as u32) {
                let mut sub = vec![];

                for i in 0..s.len() {
                    if x & (1 << i) != 0 {
                        sub.push(s[i]);
                    }
                }

                subs.insert(sub);
            }

            for sub in subs.into_iter() {
                *count.entry(sub).or_insert(0) += 1;
            }
        }

        count
            .iter()
            .filter(|&(_, c)| *c == 1)
            .map(|(sub, _)| sub.len() as i32)
            .max()
            .unwrap_or(-1)
    }
}
```
