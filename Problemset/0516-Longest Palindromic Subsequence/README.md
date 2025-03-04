# 516. Longest Palindromic Subsequence
Given a string `s`, find *the longest palindromic **subsequence**'s length in* `s`.

A **subsequence** is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

#### Example 1:
<pre>
<strong>Input:</strong> s = "bbbab"
<strong>Output:</strong> 4
<strong>Explanation:</strong> One possible longest palindromic subsequence is "bbbb".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "cbbd"
<strong>Output:</strong> 2
<strong>Explanation:</strong> One possible longest palindromic subsequence is "bb".
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s` consists only of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let mut dp = vec![vec![0; s.len()]; s.len()];

        for i in 0..s.len() {
            dp[i][i] = 1;
        }

        for size in 2..=s.len() {
            for i in 0..s.len() - size + 1 {
                dp[i][i + size - 1] =
                    dp[i + 1][i + size - 2] + (s[i] == s[i + size - 1]) as i32 * 2;
                dp[i][i + size - 1] = dp[i][i + size - 1]
                    .max(dp[i][i + size - 2])
                    .max(dp[i + 1][i + size - 1]);
            }
        }

        dp[0][s.len() - 1]
    }
}
```
