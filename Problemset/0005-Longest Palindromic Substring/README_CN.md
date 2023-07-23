# 5. 最长回文子串
给定一个字符串 `s`，找到 `s` 中最长的回文子串。你可以假设 `s` 的最大长度为 1000。

#### 示例 1:
<pre>
<strong>输入:</strong> "babad"
<strong>输出:</strong> "bab"
<strong>注意:</strong> "aba" 也是一个有效答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "cbbd"
<strong>输出:</strong> "bb"
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.into_bytes();
        let mut max_len = 1;
        let mut left = 0;

        for i in 0..s.len() {
            for j in 1..=i.min(s.len() - 1 - i) {
                if s[i - j] == s[i + j] && 1 + 2 * j > max_len {
                    max_len = 1 + 2 * j;
                    left = i - j;
                } else if s[i - j] != s[i + j] {
                    break;
                }
            }
            if i < s.len() - 1 && s[i] == s[i + 1] {
                for j in 0..=i.min(s.len() - 2 - i) {
                    if s[i - j] == s[i + 1 + j] && 2 + 2 * j > max_len {
                        max_len = 2 + 2 * j;
                        left = i - j;
                    } else if s[i - j] != s[i + 1 + j] {
                        break;
                    }
                }
            }
        }

        String::from_utf8(s[left..(left + max_len)].to_vec()).unwrap()
    }
}
```
