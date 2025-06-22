# 1910. 删除一个字符串中所有出现的给定子字符串
给你两个字符串 `s` 和 `part` ，请你对 `s` 反复执行以下操作直到 **所有** 子字符串 `part` 都被删除：
* 找到 `s` 中 **最左边** 的子字符串 `part` ，并将它从 `s` 中删除。

请你返回从 `s` 中删除所有 `part` 子字符串以后得到的剩余字符串。

一个 **子字符串** 是一个字符串中连续的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "daabcbaabcbc", part = "abc"
<strong>输出:</strong> "dab"
<strong>解释:</strong> 以下操作按顺序执行：
- s = "daabcbaabcbc" ，删除下标从 2 开始的 "abc" ，得到 s = "dabaabcbc" 。
- s = "dabaabcbc" ，删除下标从 4 开始的 "abc" ，得到 s = "dababc" 。
- s = "dababc" ，删除下标从 3 开始的 "abc" ，得到 s = "dab" 。
此时 s 中不再含有子字符串 "abc" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "axxxxyyyyb", part = "xy"
<strong>输出:</strong> "ab"
<strong>解释:</strong> 以下操作按顺序执行：
- s = "axxxxyyyyb" ，删除下标从 4 开始的 "xy" ，得到 s = "axxxyyyb" 。
- s = "axxxyyyb" ，删除下标从 3 开始的 "xy" ，得到 s = "axxyyb" 。
- s = "axxyyb" ，删除下标从 2 开始的 "xy" ，得到 s = "axyb" 。
- s = "axyb" ，删除下标从 1 开始的 "xy" ，得到 s = "ab" 。
此时 s 中不再含有子字符串 "xy" 。
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `1 <= part.length <= 1000`
* `s` 和 `part` 只包小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s = s;

        while s.contains(&part) {
            s = s.replacen(&part, "", 1);
        }

        s
    }
}
```
