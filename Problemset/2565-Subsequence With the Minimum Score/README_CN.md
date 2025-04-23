# 2565. 最少得分子序列
给你两个字符串 `s` 和 `t` 。

你可以从字符串 `t` 中删除任意数目的字符。

如果没有从字符串 `t` 中删除字符，那么得分为 `0` ，否则：
* 令 `left` 为删除字符中的最小下标。
* 令 `right` 为删除字符中的最大下标。

字符串的得分为 `right - left + 1` 。

请你返回使 `t` 成为 `s` 子序列的最小得分。

一个字符串的 **子序列** 是从原字符串中删除一些字符后（也可以一个也不删除），剩余字符不改变顺序得到的字符串。（比方说 `"ace"` 是 `"abcde"` 的子序列，但是 `"aec"` 不是）。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abacaba", t = "bzaa"
<strong>输出:</strong> 1
<strong>解释:</strong> 这个例子中，我们删除下标 1 处的字符 "z" （下标从 0 开始）。
字符串 t 变为 "baa" ，它是字符串 "abacaba" 的子序列，得分为 1 - 1 + 1 = 1 。
1 是能得到的最小得分。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "cde", t = "xyz"
<strong>输出:</strong> 3
<strong>解释:</strong> 这个例子中，我们将下标为 0， 1 和 2 处的字符 "x" ，"y" 和 "z" 删除（下标从 0 开始）。
字符串变成 "" ，它是字符串 "cde" 的子序列，得分为 2 - 0 + 1 = 3 。
3 是能得到的最小得分。
</pre>

#### 提示:
* <code>1 <= s.length, t.length <= 10<sup>5</sup></code>
* `s` 和 `t` 都只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_score(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = t.len() - 1;
        let mut matches_right = vec![(t.len(), s.len())];
        let mut ret = t.len();

        for j in (0..s.len()).rev() {
            if s[j] == t[i] {
                matches_right.push((i, j));
                ret = i;

                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }

        i = 0;

        for j in 0..s.len() {
            if s[j] == t[i] {
                while matches_right.last().unwrap_or(&(0, s.len())).1 <= j {
                    matches_right.pop();
                }
                ret = ret.min(
                    matches_right
                        .last()
                        .unwrap_or(&(t.len(), 0))
                        .0
                        .saturating_sub(i + 1),
                );

                if i == t.len() - 1 {
                    break;
                } else {
                    i += 1;
                }
            }
        }

        ret as i32
    }
}
```
