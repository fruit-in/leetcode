# 1347. 制造字母异位词的最小步骤数
给你两个长度相等的字符串 `s` 和 `t`。每一个步骤中，你可以选择将 `t` 中的 **任一字符** 替换为 **另一个字符**。

返回使 `t` 成为 `s` 的字母异位词的最小步骤数。

**字母异位词** 指字母相同，但排列不同（也可能相同）的字符串。

#### 示例 1:
<pre>
<b>输出:</b> s = "bab", t = "aba"
<b>输出:</b> 1
<b>提示:</b> 用 'b' 替换 t 中的第一个 'a'，t = "bba" 是 s 的一个字母异位词。
</pre>

#### 示例 2:
<pre>
<b>输出:</b> s = "leetcode", t = "practice"
<b>输出:</b> 5
<b>提示:</b> 用合适的字符替换 t 中的 'p', 'r', 'a', 'i' 和 'c'，使 t 变成 s 的字母异位词。
</pre>

#### 示例 3:
<pre>
<b>输出:</b> s = "anagram", t = "mangaar"
<b>输出:</b> 0
<b>提示:</b> "anagram" 和 "mangaar" 本身就是一组字母异位词。
</pre>

#### 示例 4:
<pre>
<b>输出:</b> s = "xxyyzz", t = "xxyyzz"
<b>输出:</b> 0
</pre>

#### 示例 5:
<pre>
<b>输出:</b> s = "friend", t = "family"
<b>输出:</b> 4
</pre>

#### 提示:
* `1 <= s.length <= 50000`
* `s.length == t.length`
* `s` 和 `t` 只包含小写英文字母

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut cnt = [0; 26];

        for i in 0..s.len() {
            cnt[(s[i] - b'a') as usize] += 1;
            cnt[(t[i] - b'a') as usize] -= 1;
        }

        cnt.iter().filter(|&&x| x > 0).sum()
    }
}
```
