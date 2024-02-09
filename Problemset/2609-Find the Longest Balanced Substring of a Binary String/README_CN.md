# 2609. 最长平衡子字符串
给你一个仅由 `0` 和 `1` 组成的二进制字符串 `s` 。

如果子字符串中 **所有的** `0` **都在** `1` **之前** 且其中 `0` 的数量等于 `1` 的数量，则认为 `s` 的这个子字符串是平衡子字符串。请注意，空子字符串也视作平衡子字符串。

返回  `s` 中最长的平衡子字符串长度。

子字符串是字符串中的一个连续字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "01000111"
<strong>输出:</strong> 6
<strong>解释:</strong> 最长的平衡子字符串是 "000111" ，长度为 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "00111"
<strong>输出:</strong> 4
<strong>解释:</strong> 最长的平衡子字符串是 "0011" ，长度为  4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "111"
<strong>输出:</strong> 0
<strong>解释:</strong> 除了空子字符串之外不存在其他平衡子字符串，所以答案为 0 。
</pre>

#### 提示:
* `1 <= s.length <= 50`
* `'0' <= s[i] <= '1'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count0 = (b'1' - s[0]) as i32;
        let mut count1 = 0;
        let mut ret = 0;

        for i in 1..s.len() {
            if s[i] == b'0' && s[i - 1] == b'1' {
                count0 = 0;
                count1 = 0;
            }

            if s[i] == b'0' {
                count0 += 1;
            } else if s[i] == b'1' {
                count1 += 1;
            }

            ret = ret.max(count0.min(count1) * 2);
        }

        ret
    }
}
```
