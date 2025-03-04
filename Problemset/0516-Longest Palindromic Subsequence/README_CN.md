# 516. 最长回文子序列
给你一个字符串 `s` ，找出其中最长的回文子序列，并返回该序列的长度。

子序列定义为：不改变剩余字符顺序的情况下，删除某些字符或者不删除任何字符形成的一个序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "bbbab"
<strong>输出:</strong> 4
<strong>解释:</strong> 一个可能的最长回文子序列为 "bbbb" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "cbbd"
<strong>输出:</strong> 2
<strong>解释:</strong> 一个可能的最长回文子序列为 "bb" 。
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
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
