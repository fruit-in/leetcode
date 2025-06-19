# 1392. 最长快乐前缀
**「快乐前缀」** 是在原字符串中既是 **非空** 前缀也是后缀（不包括原字符串自身）的字符串。

给你一个字符串 `s`，请你返回它的 **最长快乐前缀**。如果不存在满足题意的前缀，则返回一个空字符串 `""` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "level"
<strong>输出:</strong> "l"
<strong>解释:</strong> 不包括 s 自己，一共有 4 个前缀（"l", "le", "lev", "leve"）和 4 个后缀（"l", "el", "vel", "evel"）。最长的既是前缀也是后缀的字符串是 "l" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "ababab"
<strong>输出:</strong> "abab"
<strong>解释:</strong> "abab" 是最长的既是前缀也是后缀的字符串。题目允许前后缀在原字符串中重叠。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只含有小写英文字母

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let s = s.as_bytes();
        let mut j = 0;
        let mut next = vec![0; s.len()];

        for i in 1..s.len() {
            while j > 0 && s[i] != s[j] {
                j = next[j - 1];
            }

            if s[i] == s[j] {
                j += 1;
                next[i] = j;
            }
        }

        String::from_utf8(s[..next[s.len() - 1]].to_vec()).unwrap()
    }
}
```
