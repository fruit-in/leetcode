# 1417. 重新格式化字符串
给你一个混合了数字和字母的字符串 `s`，其中的字母均为小写英文字母。

请你将该字符串重新格式化，使得任意两个相邻字符的类型都不同。也就是说，字母后面应该跟着数字，而数字后面应该跟着字母。

请你返回 **重新格式化后** 的字符串；如果无法按要求重新格式化，则返回一个 **空字符串** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "a0b1c2"
<strong>输出:</strong> "0a1b2c"
<strong>解释:</strong> "0a1b2c" 中任意两个相邻字符的类型都不同。 "a0b1c2", "0a1b2c", "0c2a1b" 也是满足题目要求的答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "leetcode"
<strong>输出:</strong> ""
<strong>解释:</strong> "leetcode" 中只有字母，所以无法满足重新格式化的条件。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1229857369"
<strong>输出:</strong> ""
<strong>解释:</strong> "1229857369" 中只有数字，所以无法满足重新格式化的条件。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "covid2019"
<strong>输出:</strong> "c2o0v1i9d"
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = "ab123"
<strong>输出:</strong> "1a2b3"
</pre>

#### 提示:
* `1 <= s.length <= 500`
* `s` 仅由小写英文字母和/或数字组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn reformat(s: String) -> String {
        let (mut digits, mut letters): (Vec<u8>, Vec<u8>) =
            s.bytes().partition(|ch| ch.is_ascii_digit());
        let mut ret = vec![];

        let mut iterator = if digits.len() == letters.len() + 1 {
            ret.push(digits.pop().unwrap());
            letters.iter().zip(digits.iter())
        } else if digits.len() + 1 == letters.len() {
            ret.push(letters.pop().unwrap());
            digits.iter().zip(letters.iter())
        } else if digits.len() == letters.len() {
            digits.iter().zip(letters.iter())
        } else {
            [].iter().zip([].iter())
        };

        while let Some((&ch0, &ch1)) = iterator.next() {
            ret.push(ch0);
            ret.push(ch1);
        }

        String::from_utf8(ret).unwrap()
    }
}
```
