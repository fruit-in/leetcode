# 1297. 子串的最大出现次数
给你一个字符串 `s` ，请你返回满足以下条件且出现次数最大的 **任意** 子串的出现次数：

* 子串中不同字母的数目必须小于等于 `maxLetters` 。
* 子串的长度必须大于等于 minSize 且小于等于 `maxSize` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
<strong>输出:</strong> 2
<strong>解释:</strong> 子串 "aab" 在原字符串中出现了 2 次。
它满足所有的要求：2 个不同的字母，长度为 3 （在 minSize 和 maxSize 范围内）。
</pre>
子串 "aaa" 在原字符串中出现了 2 次，且它们有重叠部分。

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 子串 "aaa" 在原字符串中出现了 2 次，且它们有重叠部分。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aabcabcab", maxLetters = 2, minSize = 2, maxSize = 3
<strong>输出:</strong> 3
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "abcde", maxLetters = 2, minSize = 3, maxSize = 3
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `1 <= maxLetters <= 26`
* `1 <= minSize <= maxSize <= min(26, s.length)`
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let s = s.as_bytes();
        let min_size = min_size as usize;
        let mut letter_count = [0; 26];
        let mut unique_count = 0;
        let mut substring_count = HashMap::new();

        for i in 0..min_size {
            letter_count[(s[i] - b'a') as usize] += 1;
            unique_count += (letter_count[(s[i] - b'a') as usize] == 1) as i32;
        }
        if unique_count <= max_letters {
            substring_count.insert(&s[..min_size], 1);
        }

        for i in min_size..s.len() {
            letter_count[(s[i - min_size] - b'a') as usize] -= 1;
            unique_count -= (letter_count[(s[i - min_size] - b'a') as usize] == 0) as i32;
            letter_count[(s[i] - b'a') as usize] += 1;
            unique_count += (letter_count[(s[i] - b'a') as usize] == 1) as i32;
            if unique_count <= max_letters {
                *substring_count.entry(&s[i - min_size + 1..=i]).or_insert(0) += 1;
            }
        }

        *substring_count.values().max().unwrap_or(&0)
    }
}
```
