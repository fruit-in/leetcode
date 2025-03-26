# 2472. 不重叠回文子字符串的最大数目
给你一个字符串 `s` 和一个 **正** 整数 `k` 。

从字符串 `s` 中选出一组满足下述条件且 **不重叠** 的子字符串：
* 每个子字符串的长度 **至少** 为 `k` 。
* 每个子字符串是一个 **回文串** 。

返回最优方案中能选择的子字符串的 **最大** 数目。

**子字符串** 是字符串中一个连续的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abaccdbbd", k = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 可以选择 s = "abaccdbbd" 中斜体加粗的子字符串。"aba" 和 "dbbd" 都是回文，且长度至少为 k = 3 。
可以证明，无法选出两个以上的有效子字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "adbcda", k = 2
<strong>输出:</strong> 0
<strong>解释:</strong> 字符串中不存在长度至少为 2 的回文子字符串。
</pre>

#### 提示:
* `1 <= k <= s.length <= 2000`
* `s` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
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
