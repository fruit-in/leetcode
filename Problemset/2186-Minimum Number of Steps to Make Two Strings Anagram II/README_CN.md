# 2186. 使两字符串互为字母异位词的最少步骤数
给你两个字符串 `s` 和 `t` 。在一步操作中，你可以给 `s` 或者 `t` 追加 **任一字符** 。

返回使 `s` 和 `t` 互为 **字母异位词** 所需的最少步骤数。

**字母异位词** 指字母相同但是顺序不同（或者相同）的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leetcode", t = "coats"
<strong>输出:</strong> 7
<strong>解释:</strong>
- 执行 2 步操作，将 "as" 追加到 s = "leetcode" 中，得到 s = "leetcodeas" 。
- 执行 5 步操作，将 "leede" 追加到 t = "coats" 中，得到 t = "coatsleede" 。
"leetcodeas" 和 "coatsleede" 互为字母异位词。
总共用去 2 + 5 = 7 步。
可以证明，无法用少于 7 步操作使这两个字符串互为字母异位词。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "night", t = "thing"
<strong>输出:</strong> 0
<strong>解释:</strong> 给出的字符串已经互为字母异位词。因此，不需要任何进一步操作。
</pre>

#### 提示:
* <code>1 <= s.length, t.length <= 2 * 10<sup>5</sup></code>
* `s` 和 `t` 由小写英文字符组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut count = [(0_i32, 0_i32); 26];

        s.bytes().for_each(|b| count[(b - b'a') as usize].0 += 1);
        t.bytes().for_each(|b| count[(b - b'a') as usize].1 += 1);

        count.into_iter().map(|(x, y)| (x - y).abs()).sum()
    }
}
```
