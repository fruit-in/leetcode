# 1745. Palindrome Partitioning IV
Given a string `s`, return `true` *if it is possible to split the string* `s` *into three **non-empty** palindromic substrings. Otherwise, return* `false`.

A string is said to be palindrome if it the same string when reversed.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abcbdd"
<strong>Output:</strong> true
<strong>Explanation:</strong> "abcbdd" = "a" + "bcb" + "dd", and all three substrings are palindromes.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "bcbddxy"
<strong>Output:</strong> false
<strong>Explanation:</strong> s cannot be split into 3 palindromes.
</pre>

#### Constraints:
* `3 <= s.length <= 2000`
* `s` consists only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let mut is_palindrome = vec![vec![false; s.len()]; s.len()];

        for r in 0..s.len() {
            for l in 0..=r {
                is_palindrome[l][r] = s[l] == s[r];
                if is_palindrome[l][r] && l + 2 < r {
                    is_palindrome[l][r] = is_palindrome[l + 1][r - 1];
                }
            }
        }

        for l in 1..s.len() - 1 {
            for r in l..s.len() - 1 {
                if is_palindrome[0][l - 1]
                    && is_palindrome[l][r]
                    && is_palindrome[r + 1][s.len() - 1]
                {
                    return true;
                }
            }
        }

        false
    }
}
```
