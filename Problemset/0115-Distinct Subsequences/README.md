# 115. Distinct Subsequences
Given two strings s and t, return *the number of distinct **subsequences** of* s *which equals* t.

The test cases are generated so that the answer fits on a 32-bit signed integer.

#### Example 1:
<pre>
<strong>Input:</strong> s = "rabbbit", t = "rabbit"
<strong>Output:</strong> 3
<strong>Explanation:</strong>
As shown below, there are 3 ways you can generate "rabbit" from s.
rabbbit
rabbbit
rabbbit
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "babgbag", t = "bag"
<strong>Output:</strong> 5
<strong>Explanation:</strong>
As shown below, there are 5 ways you can generate "bag" from s.
babgbag
babgbag
babgbag
babgbag
babgbag
</pre>

#### Constraints:
* `1 <= s.length, t.length <= 1000`
* `s` and `t` consist of English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![0; t.len()]; s.len()];

        for i in 0..s.len() {
            for j in 0..t.len().min(i + 1) {
                if i > 0 {
                    dp[i][j] = dp[i - 1][j];
                }
                if s[i] == t[j] {
                    if j == 0 {
                        dp[i][j] += 1;
                    } else {
                        dp[i][j] = (dp[i][j] + dp[i - 1][j - 1]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[s.len() - 1][t.len() - 1]
    }
}
```
