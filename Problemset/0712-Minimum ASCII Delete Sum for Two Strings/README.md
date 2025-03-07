# 712. Minimum ASCII Delete Sum for Two Strings
Given two strings `s1` and `s2`, return *the lowest **ASCII** sum of deleted characters to make two strings equal*.

#### Example 1:
<pre>
<strong>Input:</strong> s1 = "sea", s2 = "eat"
<strong>Output:</strong> 231
<strong>Explanation:</strong> Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
Deleting "t" from "eat" adds 116 to the sum.
At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s1 = "delete", s2 = "leet"
<strong>Output:</strong> 403
<strong>Explanation:</strong> Deleting "dee" from "delete" to turn the string into "let",
adds 100[d] + 101[e] + 101[e] to the sum.
Deleting "e" from "leet" adds 101[e] to the sum.
At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.
</pre>

#### Constraints:
* `1 <= s1.length, s2.length <= 1000`
* `s1` and `s2` consist of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let mut dp = vec![vec![i32::MAX; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = 0;

        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                if i > 0 && j > 0 && s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                }
                if i > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j] + s1[i - 1] as i32);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i][j - 1] + s2[j - 1] as i32);
                }
            }
        }

        dp[s1.len()][s2.len()]
    }
}
```
