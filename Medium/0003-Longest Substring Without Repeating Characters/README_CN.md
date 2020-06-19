# 3. 无重复字符的最长子串
给定一个字符串，请你找出其中不含有重复字符的 **最长子串** 的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> "abcabcbb"
<strong>输出:</strong> 3
<strong>解释:</strong> 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "bbbbb"
<strong>输出:</strong> 1
<strong>解释:</strong> 因为无重复字符的最长子串是 "b"，所以其长度为 1。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "pwwkew"
<strong>输出:</strong> 3
<strong>解释:</strong> 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 <strong>子串</strong> 的长度，"pwke" 是一个<i>子序列</i>，不是子串。
</pre>

## 题解 (Rust)

### 1. 滑动窗口
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut indices = HashMap::new();
        let mut ret = 0;
        let mut i = 0;

        for j in 0..s.len() {
            match indices.insert(s[j], j) {
                Some(x) => i = i.max(x + 1),
                None => (),
            }
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
```
