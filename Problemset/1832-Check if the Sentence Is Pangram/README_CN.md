# 1832. 判断句子是否为全字母句
**全字母句** 指包含英语字母表中每个字母至少一次的句子。

给你一个仅由小写英文字母组成的字符串 `sentence` ，请你判断 `sentence` 是否为 **全字母句** 。

如果是，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> sentence = "thequickbrownfoxjumpsoverthelazydog"
<strong>输出:</strong> true
<strong>解释:</strong> sentence 包含英语字母表中每个字母至少一次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> sentence = "leetcode"
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= sentence.length <= 1000`
* `sentence` 由小写英语字母组成

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        sentence.bytes().fold(0, |acc, c| match c {
            b'A'..=b'Z' => acc | (1 << (c - b'A')),
            b'a'..=b'z' => acc | (1 << (c - b'a')),
            _ => acc,
        }) == (1 << 26) - 1
    }
}
```
