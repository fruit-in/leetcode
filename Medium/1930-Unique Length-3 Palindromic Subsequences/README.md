# 1930. Unique Length-3 Palindromic Subsequences
Given a string `s`, return *the number of **unique palindromes of length three** that are a **subsequence** of* `s`.

Note that even if there are multiple ways to obtain the same subsequence, it is still only counted **once**.

A **palindrome** is a string that reads the same forwards and backwards.

A **subsequence** of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
* For example, `"ace"` is a subsequence of `"abcde"`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aabca"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The 3 palindromic subsequences of length 3 are:
- "aba" (subsequence of "aabca")
- "aaa" (subsequence of "aabca")
- "aca" (subsequence of "aabca")
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "adc"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no palindromic subsequences of length 3 in "adc".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "bbcbaba"
<strong>Output:</strong> 4
<strong>Explanation:</strong> The 4 palindromic subsequences of length 3 are:
- "bbb" (subsequence of "bbcbaba")
- "bcb" (subsequence of "bbcbaba")
- "bab" (subsequence of "bbcbaba")
- "aba" (subsequence of "bbcbaba")
</pre>

#### Constraints:
* <code>3 <= s.length <= 10<sup>5</sup></code>
* `s` consists of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ret = 0;

        for c in b'a'..=b'z' {
            let mut mask = 0_i32;

            for i in s.iter().position(|&x| x == c).unwrap_or(0) + 1
                ..s.iter().rposition(|&x| x == c).unwrap_or(0)
            {
                mask |= 1 << (s[i] - b'a');
            }

            ret += mask.count_ones();
        }

        ret as i32
    }
}
```
