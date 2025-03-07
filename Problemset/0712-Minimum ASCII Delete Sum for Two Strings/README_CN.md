# 712. 两个字符串的最小ASCII删除和
给定两个字符串`s1` 和 `s2`，返回 *使两个字符串相等所需删除字符的 **ASCII** 值的最小和* 。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "sea", s2 = "eat"
<strong>输出:</strong> 231
<strong>解释:</strong> 在 "sea" 中删除 "s" 并将 "s" 的值(115)加入总和。
在 "eat" 中删除 "t" 并将 116 加入总和。
结束时，两个字符串相等，115 + 116 = 231 就是符合条件的最小和。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "delete", s2 = "leet"
<strong>输出:</strong> 403
<strong>解释:</strong> 在 "delete" 中删除 "dee" 字符串变成 "let"，
将 100[d]+101[e]+101[e] 加入总和。在 "leet" 中删除 "e" 将 101[e] 加入总和。
结束时，两个字符串都等于 "let"，结果即为 100+101+101+101 = 403 。
如果改为将两个字符串转换为 "lee" 或 "eet"，我们会得到 433 或 417 的结果，比答案更大。
</pre>

#### 提示:
* `1 <= s1.length, s2.length <= 1000`
* `s1` 和 `s2` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
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
