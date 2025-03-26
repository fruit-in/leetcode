# 2472. Maximum Number of Non-overlapping Palindrome Substrings
You are given a string `s` and a **positive** integer `k`.

Select a set of **non-overlapping** substrings from the string `s` that satisfy the following conditions:
* The **length** of each substring is **at least** `k`.
* Each substring is a **palindrome**.

Return *the **maximum** number of substrings in an optimal selection*.

A **substring** is a contiguous sequence of characters within a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abaccdbbd", k = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can select the substrings underlined in s = "abaccdbbd". Both "aba" and "dbbd" are palindromes and have a length of at least k = 3.
It can be shown that we cannot find a selection with more than two valid substrings.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "adbcda", k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no palindrome substring of length at least 2 in the string.
</pre>

#### Constraints:
* `1 <= k <= s.length <= 2000`
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        if k == 1 {
            return s.len() as i32;
        }

        let s = s.as_bytes();
        let k = k as usize;
        let mut dp = vec![0; s.len()];

        for i in k - 1..s.len() {
            dp[i] = dp[i - 1];
            if (0..k / 2).all(|j| s[i - k + 1 + j] == s[i - j]) {
                dp[i] = dp[i].max(dp[i.saturating_sub(k)] + 1);
            } else if i >= k && (0..(k + 1) / 2).all(|j| s[i - k + j] == s[i - j]) {
                dp[i] = dp[i].max(dp[i.saturating_sub(k + 1)] + 1);
            }
        }

        *dp.last().unwrap()
    }
}
```
