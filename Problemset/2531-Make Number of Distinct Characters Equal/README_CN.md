# 2531. 使字符串中不同字符的数目相等
给你两个下标从 **0** 开始的字符串 `word1` 和 `word2` 。

一次 **移动** 由以下两个步骤组成：
* 选中两个下标 `i` 和 `j` ，分别满足 `0 <= i < word1.length` 和 `0 <= j < word2.length` ，
* 交换 `word1[i]` 和 `word2[j]` 。

如果可以通过 **恰好一次** 移动，使 `word1` 和 `word2` 中不同字符的数目相等，则返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = "ac", word2 = "b"
<strong>输出:</strong> false
<strong>解释:</strong> 交换任何一组下标都会导致第一个字符串中有 2 个不同的字符，而在第二个字符串中只有 1 个不同字符。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = "abcc", word2 = "aab"
<strong>输出:</strong> true
<strong>解释:</strong> 交换第一个字符串的下标 2 和第二个字符串的下标 0 。之后得到 word1 = "abac" 和 word2 = "cab" ，各有 3 个不同字符。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word1 = "abcde", word2 = "fghij"
<strong>输出:</strong> true
<strong>解释:</strong> 无论交换哪一组下标，两个字符串中都会有 5 个不同字符。
</pre>

#### 提示:
* <code>1 <= word1.length, word2.length <= 10<sup>5</sup></code>
* `word1` 和 `word2` 仅由小写英文字母组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_it_possible(word1: String, word2: String) -> bool {
        let mut count1 = [0; 26];
        let mut count2 = [0; 26];
        let mut distinct1 = 0;
        let mut distinct2 = 0;

        for c in word1.bytes() {
            count1[(c - b'a') as usize] += 1;
            if count1[(c - b'a') as usize] == 1 {
                distinct1 += 1;
            }
        }
        for c in word2.bytes() {
            count2[(c - b'a') as usize] += 1;
            if count2[(c - b'a') as usize] == 1 {
                distinct2 += 1;
            }
        }

        for i in 0..26 {
            if count1[i] == 0 {
                continue;
            }

            for j in 0..26 {
                if count2[j] == 0 {
                    continue;
                }
                if i == j {
                    if distinct1 == distinct2 {
                        return true;
                    } else {
                        continue;
                    }
                }

                let mut tmp1 = distinct1;
                let mut tmp2 = distinct2;

                if count1[i] == 1 {
                    tmp1 -= 1;
                }
                if count2[i] == 0 {
                    tmp2 += 1;
                }
                if count1[j] == 0 {
                    tmp1 += 1;
                }
                if count2[j] == 1 {
                    tmp2 -= 1;
                }

                if tmp1 == tmp2 {
                    return true;
                }
            }
        }

        false
    }
}
```
