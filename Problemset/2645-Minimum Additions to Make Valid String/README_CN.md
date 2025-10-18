# 2645. 构造有效字符串的最少插入数
给你一个字符串 `word` ，你可以向其中任何位置插入 "a"、"b" 或 "c" 任意次，返回使 `word` **有效** 需要插入的最少字母数。

如果字符串可以由 "abc" 串联多次得到，则认为该字符串 **有效** 。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "b"
<strong>输出:</strong> 2
<strong>解释:</strong> 在 "b" 之前插入 "a" ，在 "b" 之后插入 "c" 可以得到有效字符串 "abc" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "aaa"
<strong>输出:</strong> 6
<strong>解释:</strong> 在每个 "a" 之后依次插入 "b" 和 "c" 可以得到有效字符串 "abcabcabc" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word = "abc"
<strong>输出:</strong> 0
<strong>解释:</strong> word 已经是有效字符串，不需要进行修改。
</pre>

#### 提示:
* `1 <= word.length <= 50`
* `word` 仅由字母 "a"、"b" 和 "c" 组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut want = b'a';
        let mut ret = 0;

        for ch in word.bytes() {
            ret += (ch + 3 - want) as i32 % 3;
            want = (ch - b'a' + 1) % 3 + b'a';
        }

        ret += 2 - (want - b'a' + 2) as i32 % 3;

        ret
    }
}
```
