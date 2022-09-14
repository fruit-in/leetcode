# 1930. 长度为 3 的不同回文子序列
给你一个字符串 `s` ，返回 `s` 中 **长度为 3 的不同回文子序列** 的个数。

即便存在多种方法来构建相同的子序列，但相同的子序列只计数一次。

**回文** 是正着读和反着读一样的字符串。

**子序列** 是由原字符串删除其中部分字符（也可以不删除）且不改变剩余字符之间相对顺序形成的一个新字符串。
* 例如，`"ace"` 是 `"abcde"` 的一个子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aabca"
<strong>输出:</strong> 3
<strong>解释:</strong> 长度为 3 的 3 个回文子序列分别是：
- "aba" ("aabca" 的子序列)
- "aaa" ("aabca" 的子序列)
- "aca" ("aabca" 的子序列)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "adc"
<strong>输出:</strong> 0
<strong>解释:</strong> "adc" 不存在长度为 3 的回文子序列。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "bbcbaba"
<strong>输出:</strong> 4
<strong>解释:</strong> 长度为 3 的 4 个回文子序列分别是：
- "bbb" ("bbcbaba" 的子序列)
- "bcb" ("bbcbaba" 的子序列)
- "bab" ("bbcbaba" 的子序列)
- "aba" ("bbcbaba" 的子序列)
</pre>

#### 提示:
* <code>3 <= s.length <= 10<sup>5</sup></code>
* `s` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ret = 0;

        for c in b'a'..=b'z' {
            let mut mask = 0_i32;

            for i in s.iter().position(|&x| x == c).unwrap_or(0) + 1
                ..s.iter().rposition(|&x| x == c).unwrap_or(0)
            {
                mask |= 1 << (s[i] - b'a');
            }

            ret += mask.count_ones();
        }

        ret as i32
    }
}
```
