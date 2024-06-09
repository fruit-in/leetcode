# 1839. 所有元音按顺序排布的最长子字符串
当一个字符串满足如下条件时，我们称它是 **美丽的** ：

* 所有 5 个英文元音字母（`'a'` ，`'e'` ，`'i'` ，`'o'` ，`'u'`）都必须 **至少** 出现一次。
* 这些元音字母的顺序都必须按照 **字典序** 升序排布（也就是说所有的 `'a'` 都在 `'e'` 前面，所有的 `'e'` 都在 `'i'` 前面，以此类推）

比方说，字符串 `"aeiou"` 和 `"aaaaaaeiiiioou"` 都是 **美丽的** ，但是 `"uaeio"` ，`"aeoiu"` 和 `"aaaeeeooo"` **不是美丽的** 。

给你一个只包含英文元音字母的字符串 `word` ，请你返回 `word` 中 **最长美丽子字符串的长度** 。如果不存在这样的子字符串，请返回 `0` 。

**子字符串** 是字符串中一个连续的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "aeiaaioaaaaeiiiiouuuooaauuaeiu"
<strong>输出:</strong> 13
<strong>解释:</strong> 最长子字符串是 "aaaaeiiiiouuu" ，长度为 13 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "aeeeiiiioooauuuaeiou"
<strong>输出:</strong> 5
<strong>解释:</strong> 最长子字符串是 "aeiou" ，长度为 5 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word = "a"
<strong>输出:</strong> 0
<strong>解释:</strong> 没有美丽子字符串，所以返回 0 。
</pre>

#### 提示:
* <code>1 <= word.length <= 5 * 10<sup>5</sup></code>
* `word` 只包含字符 `'a'`，`'e'`，`'i'`，`'o'` 和 `'u'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut vowels = "_aeiou_".as_bytes();
        let mut i = 0;
        let mut count = 0;
        let mut ret = 0;

        for c in word.bytes().chain(std::iter::once(b' ')) {
            if c == vowels[i] {
                count += 1;
            } else if c == vowels[i + 1] {
                i += 1;
                count += 1;
            } else {
                if i == 5 {
                    ret = ret.max(count);
                }
                i = (c == b'a') as usize;
                count = i as i32;
            }
        }

        ret
    }
}
```
