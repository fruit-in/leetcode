# 820. 单词的压缩编码
单词数组 `words` 的 **有效编码** 由任意助记字符串 `s` 和下标数组 `indices` 组成，且满足：
* `words.length == indices.length`
* 助记字符串 `s` 以 `'#'` 字符结尾
* 对于每个下标 `indices[i]` ，`s` 的一个从 `indices[i]` 开始、到下一个 `'#'` 字符结束（但不包括 `'#'`）的 **子字符串** 恰好与 `words[i]` 相等

给你一个单词数组 `words` ，返回成功对 `words` 进行编码的最小助记字符串 `s` 的长度 。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["time", "me", "bell"]
<strong>输出:</strong> 10
<strong>解释:</strong> 一组有效编码为 s = "time#bell#" 和 indices = [0, 2, 5] 。
words[0] = "time" ，s 开始于 indices[0] = 0 到下一个 '#' 结束的子字符串，如加粗部分所示 "time#bell#"
words[1] = "me" ，s 开始于 indices[1] = 2 到下一个 '#' 结束的子字符串，如加粗部分所示 "time#bell#"
words[2] = "bell" ，s 开始于 indices[2] = 5 到下一个 '#' 结束的子字符串，如加粗部分所示 "time#bell#"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["t"]
<strong>输出:</strong> 2
<strong>解释:</strong> 一组有效编码为 s = "t#" 和 indices = [0] 。
</pre>

#### 提示:
* `1 <= words.length <= 2000`
* `1 <= words[i].length <= 7`
* `words[i]` 仅由小写字母组成

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let words = words.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        let mut suffixes = HashSet::new();

        for w in &words {
            for i in 1..w.len() {
                suffixes.insert(w.get(i..).unwrap());
            }
        }

        words
            .difference(&suffixes)
            .map(|w| w.len() as i32 + 1)
            .sum()
    }
}
```
