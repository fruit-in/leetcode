# 2063. 所有子字符串中的元音
给你一个字符串 `word` ，返回 `word` 的所有子字符串中 **元音的总数** ，元音是指 `'a'`、`'e'`、`'i'`、`'o'` 和 `'u'` 。

**子字符串** 是字符串中一个连续（非空）的字符序列。

**注意：**由于对 `word` 长度的限制比较宽松，答案可能超过有符号 32 位整数的范围。计算时需当心。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "aba"
<strong>输出:</strong> 6
<strong>解释:</strong>
所有子字符串是："a"、"ab"、"aba"、"b"、"ba" 和 "a" 。
- "b" 中有 0 个元音
- "a"、"ab"、"ba" 和 "a" 每个都有 1 个元音
- "aba" 中有 2 个元音
因此，元音总数 = 0 + 1 + 1 + 1 + 1 + 2 = 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "abc"
<strong>输出:</strong> 3
<strong>解释:</strong>
所有子字符串是："a"、"ab"、"abc"、"b"、"bc" 和 "c" 。
- "a"、"ab" 和 "abc" 每个都有 1 个元音
- "b"、"bc" 和 "c" 每个都有 0 个元音
因此，元音总数 = 1 + 1 + 1 + 0 + 0 + 0 = 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word = "ltcd"
<strong>输出:</strong> 0
<strong>解释:</strong> "ltcd" 的子字符串均不含元音。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> word = "noosabasboosa"
<strong>输出:</strong> 237
<strong>解释:</strong> 所有子字符串中共有 237 个元音。
</pre>

#### 提示:
* <code>1 <= word.length <= 10<sup>5</sup></code>
* `word` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let word = word.as_bytes();
        let mut x = 0;
        let mut ret = 0;

        for i in 0..word.len() {
            if [b'a', b'e', b'i', b'o', b'u'].contains(&word[i]) {
                x += i as i64 + 1;
            }

            ret += x;
        }

        ret
    }
}
```
