# 1653. Minimum Deletions to Make String Balanced
You are given a string `s` consisting only of characters `'a'` and `'b'`.

You can delete any number of characters in `s` to make `s` **balanced**. `s` is **balanced** if there is no pair of indices `(i,j)` such that `i < j` and `s[i] = 'b'` and `s[j]= 'a'`.

Return *the **minimum** number of deletions needed to make* `s` ***balanced***.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aababbab"
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can either:
Delete the characters at 0-indexed positions 2 and 6 ("aababbab" -> "aaabbb"), or
Delete the characters at 0-indexed positions 3 and 6 ("aababbab" -> "aabbbb").
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "bbaaaaabb"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The only solution is to delete the first two characters.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s[i]` is `'a'` or `'b'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut count = s.chars().filter(|&c| c == 'a').count() as i32;
        let mut ret = count;

        for c in s.chars() {
            count += (c == 'b') as i32 - (c == 'a') as i32;
            ret = ret.min(count);
        }

        ret
    }
}
```
