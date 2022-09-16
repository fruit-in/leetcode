# 316. Remove Duplicate Letters
Given a string `s`, remove duplicate letters so that every letter appears once and only once. You must make sure your result is **the smallest in lexicographical order** among all possible results.

#### Example 1:
<pre>
<strong>Input:</strong> s = "bcabc"
<strong>Output:</strong> "abc"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "cbacdcbc"
<strong>Output:</strong> "acdb"
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s` consists of lowercase English letters.

**Note:** This question is the same as 1081: https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let s = s.as_bytes();
        let mut mask = 0;
        let mut masks = vec![0; s.len()];
        let mut indices = vec![vec![]; 26];
        let mut i = 0;
        let mut ret = vec![];

        for j in (0..s.len()).rev() {
            masks[j] = mask;
            mask |= 1 << (s[j] - b'a');
            indices[(s[j] - b'a') as usize].push(j);
        }

        while mask != 0 {
            for j in (0..=26).filter(|x| mask & (1 << x) != 0) {
                let new_mask = mask ^ (1 << j);
                let k = match indices[j].binary_search_by(|x| i.cmp(x)) {
                    Ok(y) => indices[j][y],
                    Err(y) => indices[j][y - 1],
                };

                if masks[k] & new_mask == new_mask {
                    mask = new_mask;
                    i = k;
                    ret.push(b'a' + j as u8);
                    break;
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
