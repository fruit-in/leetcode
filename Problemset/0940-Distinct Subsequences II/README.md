# 940. Distinct Subsequences II
Given a string s, return *the number of **distinct non-empty subsequences** of* `s`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A **subsequence** of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., `"ace"` is a subsequence of `"abcde"` while `"aec"` is not.

#### Example 1:
<pre>
<strong>Input:</strong> s = "abc"
<strong>Output:</strong> 7
<strong>Explanation:</strong> The 7 distinct subsequences are "a", "b", "c", "ab", "ac", "bc", and "abc".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aba"
<strong>Output:</strong> 6
<strong>Explanation:</strong> The 6 distinct subsequences are "a", "b", "ab", "aa", "ba", and "aba".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aaa"
<strong>Output:</strong> 3
<strong>Explanation:</strong> The 3 distinct subsequences are "a", "aa" and "aaa".
</pre>

#### Constraints:
* `1 <= s.length <= 2000`
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = s.len();
        let mut last_index = [n; 26];
        let mut dp = vec![0_i32; n + 1];
        dp[n] = -1;

        for (i, c) in s.bytes().map(|c| (c - b'a') as usize).enumerate() {
            dp[i + 1] = (dp[i] * 2 - dp[last_index[c]]).rem_euclid(MOD);
            last_index[c] = i;
        }

        *dp.last().unwrap()
    }
}
```
