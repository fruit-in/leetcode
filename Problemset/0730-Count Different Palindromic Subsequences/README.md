# 730. Count Different Palindromic Subsequences
Given a string s, return *the number of different non-empty palindromic subsequences in* `s`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

A subsequence of a string is obtained by deleting zero or more characters from the string.

A sequence is palindromic if it is equal to the sequence reversed.

Two sequences <code>a<sub>1</sub>, a<sub>2</sub>, ...</code> and <code>b<sub>1</sub>, b<sub>2</sub>, ...</code> are different if there is some `i` for which <code>a<sub>i</sub> != b<sub>i</sub></code>.

#### Example 1:
<pre>
<strong>Input:</strong> s = "bccb"
<strong>Output:</strong> 6
<strong>Explanation:</strong> The 6 different non-empty palindromic subsequences are 'b', 'c', 'bb', 'cc', 'bcb', 'bccb'.
Note that 'bcb' is counted only once, even though it occurs twice.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
<strong>Output:</strong> 104860361
<strong>Explanation:</strong> There are 3104860382 different non-empty palindromic subsequences, which is 104860361 modulo 10<sup>9</sup> + 7.
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `s[i]` is either `'a'`, `'b'`, `'c'`, or `'d'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0_i32; n]; n];

        for i in (0..n).rev() {
            let mut first = [n; 4];
            let mut last = [n; 4];

            for j in i..n {
                let k = (s[j] - b'a') as usize;
                dp[i][j] = dp[i][j.saturating_sub(1)];

                if first[k] == n {
                    dp[i][j] += 1;
                    first[k] = j;
                } else if first[k] == last[k] {
                    dp[i][j] += dp[first[k] + 1][j - 1] + 1;
                } else {
                    dp[i][j] += dp[first[k] + 1][j - 1] - dp[first[k] + 1][last[k] - 1];
                }

                dp[i][j] = dp[i][j].rem_euclid(1_000_000_007);
                last[k] = j;
            }
        }

        dp[0][n - 1]
    }
}
```
