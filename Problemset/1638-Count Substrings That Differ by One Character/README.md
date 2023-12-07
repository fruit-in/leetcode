# 1638. Count Substrings That Differ by One Character
Given two strings `s` and `t`, find the number of ways you can choose a non-empty substring of `s` and replace a **single character** by a different character such that the resulting substring is a substring of `t`. In other words, find the number of substrings in `s` that differ from some substring in `t` by **exactly** one character.

For example, the underlined substrings in <code>"<u>compute</u>r"</code> and <code>"<u>computa</u>tion"</code> only differ by the `'e'`/`'a'`, so this is a valid way.

Return *the number of substrings that satisfy the condition above*.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aba", t = "baba"
<strong>Output:</strong> 6
<strong>Explanation:</strong> The following are the pairs of substrings from s and t that differ by exactly 1 character:
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
The underlined portions are the substrings that are chosen from s and t.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "ab", t = "bb"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The following are the pairs of substrings from s and t that differ by 1 character:
("ab", "bb")
("ab", "bb")
("ab", "bb")
The underlined portions are the substrings that are chosen from s and t.
</pre>

#### Constraints:
* `1 <= s.length, t.length <= 100`
* `s` and `t` consist of lowercase English letters only.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut ret = 0;

        for i in 0..s.len() {
            for j in 0..t.len() {
                let mut count = 0;

                for k in 0..(s.len() - i).min(t.len() - j) {
                    if s[i + k] != t[j + k] {
                        count += 1;
                    }
                    if count > 1 {
                        break;
                    } else if count == 1 {
                        ret += 1;
                    }
                }
            }
        }

        ret
    }
}
```
