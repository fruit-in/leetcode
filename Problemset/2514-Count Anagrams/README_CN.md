# 2514. 统计同位异构字符串数目
给你一个字符串 `s` ，它包含一个或者多个单词。单词之间用单个空格 `' '` 隔开。

如果字符串 `t` 中第 `i` 个单词是 `s` 中第 `i` 个单词的一个 **排列** ，那么我们称字符串 `t` 是字符串 `s` 的同位异构字符串。

* 比方说，`"acb dfe"` 是 `"abc def"` 的同位异构字符串，但是 `"def cab"` 和 `"adc bef"` 不是。

请你返回 `s` 的同位异构字符串的数目，由于答案可能很大，请你将它对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "too hot"
<strong>输出:</strong> 18
<strong>解释:</strong> 输入字符串的一些同位异构字符串为 "too hot" ，"oot hot" ，"oto toh" ，"too toh" 以及 "too oht" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aa"
<strong>输出:</strong> 1
<strong>解释:</strong> 输入字符串只有一个同位异构字符串。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含小写英文字母和空格 `' '` 。
* 相邻单词之间由单个空格隔开。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        fn power(x: i64, exp: u32) -> i64 {
            if exp == 0 {
                1
            } else if exp % 2 == 0 {
                power(x, exp / 2).pow(2) % 1_000_000_007
            } else {
                x * power(x, exp - 1) % 1_000_000_007
            }
        }

        let mut factorials = vec![1];
        let mut ret = 1_i64;

        for word in s.split_whitespace() {
            let mut count = [0; 26];
            let mut length = 0;

            for c in word.bytes() {
                count[(c - b'a') as usize] += 1;
            }

            for c in count {
                length += c;
                while length >= factorials.len() {
                    let x = (*factorials.last().unwrap() * factorials.len() as i64) % 1_000_000_007;
                    factorials.push(x);
                }
                ret = (ret * factorials[length]) % 1_000_000_007;
                ret = (ret * power(factorials[c], 1_000_000_005)) % 1_000_000_007;
                ret = (ret * power(factorials[length - c], 1_000_000_005)) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
```
