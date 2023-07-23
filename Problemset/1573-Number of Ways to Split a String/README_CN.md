# 1573. 分割字符串的方案数
给你一个二进制串 `s`  （一个只包含 0 和 1 的字符串），我们可以将 `s` 分割成 3 个 **非空** 字符串 s1, s2, s3 （s1 + s2 + s3 = s）。

请你返回分割 `s` 的方案数，满足 s1，s2 和 s3 中字符 '1' 的数目相同。

由于答案可能很大，请将它对 10^9 + 7 取余后返回。

#### 示例 1:
<pre>
<b>输入:</b> s = "10101"
<b>输出:</b> 4
<b>解释:</b> 总共有 4 种方法将 s 分割成含有 '1' 数目相同的三个子字符串。
"1|010|1"
"1|01|01"
"10|10|1"
"10|1|01"
</pre>

#### 示例 2:
<pre>
<b>输入:</b> s = "1001"
<b>输出:</b> 0
</pre>

#### 示例 3:
<pre>
<b>输入:</b> s = "0000"
<b>输出:</b> 3
<b>解释:</b> 总共有 3 种分割 s 的方法。
"0|0|00"
"0|00|0"
"00|0|0"
</pre>

#### 示例 4:
<pre>
<b>输入:</b> s = "100100010100110"
<b>输出:</b> 12
</pre>

#### 提示:
* `s[i] == '0'` 或者 `s[i] == '1'`
* `3 <= s.length <= 10^5`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let mut indices = s
            .bytes()
            .enumerate()
            .filter(|&(i, c)| c == b'1')
            .map(|(i, c)| i as i32)
            .collect::<Vec<_>>();

        if indices.len() % 3 != 0 {
            return 0;
        }

        if indices.is_empty() {
            return ((s.len() - 2) as i64 * (s.len() - 1) as i64 / 2 % 1000000007) as i32;
        }

        let third = indices.len() / 3;
        let split_12 = (indices[third] - indices[third - 1]) as i64;
        let split_23 = (indices[2 * third] - indices[2 * third - 1]) as i64;

        (split_12 * split_23 % 1000000007) as i32
    }
}
```
