# 1844. 将所有数字用字符替换
给你一个下标从 **0** 开始的字符串 `s` ，它的 **偶数** 下标处为小写英文字母，**奇数** 下标处为数字。

定义一个函数 `shift(c, x)` ，其中 `c` 是一个字符且 `x` 是一个数字，函数返回字母表中 `c` 后面第 `x` 个字符。
* 比方说，`shift('a', 5) = 'f'` 和 `shift('x', 0) = 'x'` 。

对于每个 **奇数** 下标 `i` ，你需要将数字 `s[i]` 用 `shift(s[i-1], s[i])` 替换。

请你替换所有数字以后，将字符串 `s` 返回。题目 **保证** `shift(s[i-1], s[i])` 不会超过 `'z'` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "a1c1e1"
<strong>输出:</strong> "abcdef"
<strong>解释:</strong> 数字被替换结果如下：
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('c',1) = 'd'
- s[5] -> shift('e',1) = 'f'
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "a1b2c3d4e"
<strong>输出:</strong> "abbdcfdhe"
<strong>解释:</strong> 数字被替换结果如下：
- s[1] -> shift('a',1) = 'b'
- s[3] -> shift('b',2) = 'd'
- s[5] -> shift('c',3) = 'f'
- s[7] -> shift('d',4) = 'h'
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s` 只包含小写英文字母和数字。
* 对所有 **奇数** 下标处的 `i` ，满足 `shift(s[i-1], s[i]) <= 'z'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut s = s.into_bytes();

        for i in (1..s.len()).step_by(2) {
            s[i] += s[i - 1] - b'0';
        }

        String::from_utf8(s).unwrap()
    }
}
```
