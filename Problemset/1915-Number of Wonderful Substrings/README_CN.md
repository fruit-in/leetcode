# 1915. 最美子字符串的数目
如果某个字符串中 **至多一个** 字母出现 **奇数** 次，则称其为 **最美** 字符串。

* 例如，`"ccjjc"` 和 `"abab"` 都是最美字符串，但 `"ab"` 不是。

给你一个字符串 `word` ，该字符串由前十个小写英文字母组成（`'a'` 到 `'j'`）。请你返回 `word` 中 **最美非空子字符串** 的数目。如果同样的子字符串在 `word` 中出现多次，那么应当对 **每次出现** 分别计数。

**子字符串** 是字符串中的一个连续字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "aba"
<strong>输出:</strong> 4
<strong>解释:</strong> 4 个最美子字符串如下所示：
- "aba" -> "a"
- "aba" -> "b"
- "aba" -> "a"
- "aba" -> "aba"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "aabb"
<strong>输出:</strong> 9
<strong>解释:</strong> 9 个最美子字符串如下所示：
- "aabb" -> "a"
- "aabb" -> "aa"
- "aabb" -> "aab"
- "aabb" -> "aabb"
- "aabb" -> "a"
- "aabb" -> "abb"
- "aabb" -> "b"
- "aabb" -> "bb"
- "aabb" -> "b"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word = "he"
<strong>输出:</strong> 2
<strong>解释:</strong> 2 个最美子字符串如下所示：
- "he" -> "h"
- "he" -> "e"
</pre>

#### 提示:
* <code>1 <= word.length <= 10<sup>5</sup></code>
* `word` 由从 `'a'` 到 `'j'` 的小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let word = word.as_bytes();
        let mut count = [0; 1024];
        let mut mask = 0;
        let mut ret = 0;
        count[0] = 1;

        for i in 0..word.len() {
            mask ^= 1 << (word[i] - b'a');
            for j in 0..10 {
                ret += count[mask ^ (1 << j)];
            }
            ret += count[mask];
            count[mask] += 1;
        }

        ret
    }
}
```
