# 583. 两个字符串的删除操作
给定两个单词 `word1` 和 `word2` ，返回使得 `word1` 和  `word2` **相同**所需的**最小步数**。

**每步** 可以删除任意一个字符串中的一个字符。

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = "sea", word2 = "eat"
<strong>输出:</strong> 2
<strong>解释:</strong> 第一步将 "sea" 变为 "ea" ，第二步将 "eat "变为 "ea"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = "leetcode", word2 = "etco"
<strong>输出:</strong> 4
</pre>

#### 提示:
* `1 <= word1.length, word2.length <= 500`
* `word1` 和 `word2` 只包含小写英文字母

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let m = word1.len();
        let n = word2.len();
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if word1[i] == word2[j] {
                    if i > 0 && j > 0 {
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                    dp[i][j] += 1;
                }
                if i > 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1]);
                }
            }
        }

        (m + n - 2 * dp[m - 1][n - 1]) as i32
    }
}
```
