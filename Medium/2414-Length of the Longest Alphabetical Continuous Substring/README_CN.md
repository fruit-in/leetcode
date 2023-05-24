# 2414. 最长的字母序连续子字符串的长度
**字母序连续字符串** 是由字母表中连续字母组成的字符串。换句话说，字符串 `"abcdefghijklmnopqrstuvwxyz"` 的任意子字符串都是 **字母序连续字符串** 。

* 例如，`"abc"` 是一个字母序连续字符串，而 `"acb"` 和 `"za"` 不是。

给你一个仅由小写英文字母组成的字符串 `s` ，返回其 **最长** 的 字母序连续子字符串 的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abacaba"
<strong>输出:</strong> 2
<strong>解释:</strong> 共有 4 个不同的字母序连续子字符串 "a"、"b"、"c" 和 "ab" 。
"ab" 是最长的字母序连续子字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcde"
<strong>输出:</strong> 5
<strong>解释:</strong> "abcde" 是最长的字母序连续子字符串。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 1;
        let mut ret = 1;

        for i in 1..s.len() {
            if s[i - 1] + 1 == s[i] {
                count += 1;
                ret = ret.max(count);
            } else {
                count = 1;
            }
        }

        ret
    }
}
```
