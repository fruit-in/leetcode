# 2506. 统计相似字符串对的数目
给你一个下标从 **0** 开始的字符串数组 `words` 。

如果两个字符串由相同的字符组成，则认为这两个字符串 **相似** 。

* 例如，`"abca"` 和 `"cba"` 相似，因为它们都由字符 `'a'`、`'b'`、`'c'` 组成。
* 然而，`"abacba"` 和 `"bcfd"` 不相似，因为它们不是相同字符组成的。

请你找出满足字符串 `words[i]` 和 `words[j]` 相似的下标对 `(i, j)` ，并返回下标对的数目，其中 `0 <= i < j <= word.length - 1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["aba","aabb","abcd","bac","aabc"]
<strong>输出:</strong> 2
<strong>解释:</strong> 共有 2 对满足条件：
- i = 0 且 j = 1 ：words[0] 和 words[1] 只由字符 'a' 和 'b' 组成。
- i = 3 且 j = 4 ：words[3] 和 words[4] 只由字符 'a'、'b' 和 'c' 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["aabb","ab","ba"]
<strong>输出:</strong> 3
<strong>解释:</strong> 共有 3 对满足条件：
- i = 0 且 j = 1 ：words[0] 和 words[1] 只由字符 'a' 和 'b' 组成。
- i = 0 且 j = 2 ：words[0] 和 words[2] 只由字符 'a' 和 'b' 组成。
- i = 1 且 j = 2 ：words[1] 和 words[2] 只由字符 'a' 和 'b' 组成。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["nba","cba","dba"]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在满足条件的下标对，返回 0 。
</pre>

#### 提示:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count = HashMap::new();

        for word in &words {
            let x = word.bytes().fold(0, |acc, b| acc | (1 << (b - b'a')));

            count.entry(x).and_modify(|c| *c += 1).or_insert(1);
        }

        count.values().map(|c| c * (c - 1) / 2).sum()
    }
}
```
