# 72. Edit Distance
Given two strings `word1` and `word2`, return *the minimum number of operations required to convert `word1` to `word2`*.

You have the following three operations permitted on a word:
* Insert a character
* Delete a character
* Replace a character

#### Example 1:
<pre>
<strong>Input:</strong> word1 = "horse", word2 = "ros"
<strong>Output:</strong> 3
<strong>Explanation:</strong>
horse -> rorse (replace 'h' with 'r')
rorse -> rose (remove 'r')
rose -> ros (remove 'e')
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> word1 = "intention", word2 = "execution"
<strong>Output:</strong> 5
<strong>Explanation:</strong>
intention -> inention (remove 't')
inention -> enention (replace 'i' with 'e')
enention -> exention (replace 'n' with 'x')
exention -> exection (replace 'n' with 'c')
exection -> execution (insert 'u')
</pre>

#### Constraints:
* `0 <= word1.length, word2.length <= 500`
* `word1` and `word2` consist of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut dp = vec![vec![i32::MAX; word2.len() + 1]; word1.len() + 1];
        dp[0] = (0..=word2.len() as i32).collect();

        for i in 1..=word1.len() {
            dp[i][0] = i as i32;
            for j in 1..=word2.len() {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                }
                dp[i][j] = dp[i][j]
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j] + 1)
                    .min(dp[i - 1][j - 1] + 1);
            }
        }

        dp[word1.len()][word2.len()]
    }
}
```
