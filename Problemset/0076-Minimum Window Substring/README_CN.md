# 76. 最小覆盖子串
给你一个字符串 `s` 、一个字符串 `t` 。返回 `s` 中涵盖 `t` 所有字符的最小子串。如果 `s` 中不存在涵盖 `t` 所有字符的子串，则返回空字符串 `""` 。

#### 注意:
* 对于 `t` 中重复字符，我们寻找的子字符串中该字符数量必须不少于 `t` 中该字符数量。
* 如果 `s` 中存在这样的子串，我们保证它是唯一的答案。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "ADOBECODEBANC", t = "ABC"
<strong>输出:</strong> "BANC"
<strong>解释:</strong> 最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "a", t = "a"
<strong>输出:</strong> "a"
<strong>解释:</strong> 整个字符串 s 是最小覆盖子串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "a", t = "aa"
<strong>输出:</strong> ""
<strong>解释:</strong> t 中两个字符 'a' 均应包含在 s 的子串中，
因此没有符合条件的子字符串，返回空字符串。
</pre>

#### 提示:
* `m == s.length`
* `n == t.length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* `s` 和 `t` 由英文字母组成

**进阶：**你能设计一个在 `o(m+n)` 时间内解决此问题的算法吗？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }

        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut count_s = [0; 52];
        let mut count_t = [0; 52];
        let mut sub_indices = (0, s.len() + 1);
        let mut i = 0;
        let mut j = 0;

        while j < t.len() {
            match s[j] {
                b'A'..=b'Z' => count_s[(s[j] - b'A') as usize] += 1,
                _ => count_s[(s[j] - b'a') as usize + 26] += 1,
            }
            match t[j] {
                b'A'..=b'Z' => count_t[(t[j] - b'A') as usize] += 1,
                _ => count_t[(t[j] - b'a') as usize + 26] += 1,
            }

            j += 1;
        }

        if (0..52).all(|k| count_s[k] >= count_t[k]) {
            sub_indices = (i, j);
        }

        while j < s.len() {
            while j < s.len() && (0..52).any(|k| count_s[k] < count_t[k]) {
                match s[j] {
                    b'A'..=b'Z' => count_s[(s[j] - b'A') as usize] += 1,
                    _ => count_s[(s[j] - b'a') as usize + 26] += 1,
                }

                j += 1;
            }

            while (0..52).all(|k| count_s[k] >= count_t[k]) {
                if j - i < sub_indices.1 - sub_indices.0 {
                    sub_indices = (i, j);
                }

                match s[i] {
                    b'A'..=b'Z' => count_s[(s[i] - b'A') as usize] -= 1,
                    _ => count_s[(s[i] - b'a') as usize + 26] -= 1,
                }

                i += 1;
            }
        }

        if sub_indices.1 - sub_indices.0 > s.len() {
            return String::new();
        }

        String::from_utf8(s[sub_indices.0..sub_indices.1].to_vec()).unwrap()
    }
}
```
