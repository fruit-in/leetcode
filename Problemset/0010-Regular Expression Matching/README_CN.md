# 10. 正则表达式匹配
给你一个字符串 `s` 和一个字符规律 `p`，请你来实现一个支持 `'.'` 和 `'*'` 的正则表达式匹配。
* `'.'` 匹配任意单个字符
* `'*'` 匹配零个或多个前面的那一个元素

所谓匹配，是要涵盖 **整个** 字符串 `s` 的，而不是部分字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aa", p = "a"
<strong>输出:</strong> false
<strong>解释:</strong> "a" 无法匹配 "aa" 整个字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aa", p = "a*"
<strong>输出:</strong> true
<strong>解释:</strong> 因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "ab", p = ".*"
<strong>输出:</strong> true
<strong>解释:</strong> ".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
</pre>

#### 提示:
* `1 <= s.length <= 20`
* `1 <= p.length <= 20`
* `s` 只包含从 `a-z` 的小写字母。
* `p` 只包含从 `a-z` 的小写字母，以及字符 `.` 和 `*`。
* 保证每次出现字符 `*` 时，前面都匹配到有效的字符

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let mut stack = vec![];
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for c in p.bytes() {
            if c != b'*' {
                stack.push(c);
            } else {
                match stack.pop() {
                    Some(b'.') => stack.push(b','),
                    Some(c) if c >= b'a' => stack.push(c - b'a' + b'A'),
                    _ => (),
                }
            }
        }

        for &c in &stack {
            let mut tmp = vec![false; s.len() + 1];

            if c == b',' || c.is_ascii_uppercase() {
                tmp[0] = dp[0];
            }

            for i in 1..=s.len() {
                match c {
                    b'.' => tmp[i] = dp[i - 1],
                    b'a'..=b'z' => tmp[i] = c == s[i - 1] && dp[i - 1],
                    _ => {
                        tmp[i] = dp[i];
                        for j in (0..i).rev() {
                            if !tmp[i] && (c == b',' || c - b'A' + b'a' == s[j]) {
                                tmp[i] = dp[j];
                            } else {
                                break;
                            }
                        }
                    }
                }
            }

            dp = tmp;
        }

        dp[s.len()]
    }
}
```
