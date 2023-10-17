# 1143. 最长公共子序列
给定两个字符串 `text1` 和 `text2`，返回这两个字符串的最长 **公共子序列** 的长度。如果不存在 **公共子序列** ，返回 `0` 。

一个字符串的 **子序列** 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。

* 例如，`"ace"` 是 `"abcde"` 的子序列，但 `"aec"` 不是 `"abcde"` 的子序列。

两个字符串的 **公共子序列** 是这两个字符串所共同拥有的子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> text1 = "abcde", text2 = "ace"
<strong>输出:</strong> 3
<strong>解释:</strong> 最长公共子序列是 "ace" ，它的长度为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> text1 = "abc", text2 = "abc"
<strong>输出:</strong> 3
<strong>解释:</strong> 最长公共子序列是 "abc" ，它的长度为 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> text1 = "abc", text2 = "def"
<strong>输出:</strong> 0
<strong>解释:</strong> 两个字符串没有公共子序列，返回 0 。
</pre>

#### 提示:
* `1 <= text1.length, text2.length <= 1000`
* `text1` 和 `text2` 仅由小写英文字符组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        let mut dp = vec![vec![0; text2.len()]; text1.len()];

        for i in 0..text1.len() {
            for j in 0..text2.len() {
                if text1[i] == text2[j] {
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

        dp[text1.len() - 1][text2.len() - 1]
    }
}
```
