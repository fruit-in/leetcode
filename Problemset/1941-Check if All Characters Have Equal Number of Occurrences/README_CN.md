# 1941. 检查是否所有字符出现次数相同
给你一个字符串 `s` ，如果 `s` 是一个 **好** 字符串，请你返回 `true` ，否则请返回 `false` 。

如果 `s` 中出现过的 **所有** 字符的出现次数 **相同** ，那么我们称字符串 `s` 是 **好** 字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abacbc"
<strong>输出:</strong> true
<strong>解释:</strong> s 中出现过的字符为 'a'，'b' 和 'c' 。s 中所有字符均出现 2 次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aaabb"
<strong>输出:</strong> false
<strong>解释:</strong> s 中出现过的字符为 'a' 和 'b' 。
'a' 出现了 3 次，'b' 出现了 2 次，两者出现次数不同。
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut count = [0; 26];

        s.bytes().for_each(|c| count[(c - b'a') as usize] += 1);

        for i in 1..26 {
            if count[i] == 0 {
                count[i] = count[i - 1];
            } else if count[i - 1] != 0 && count[i] != count[i - 1] {
                return false;
            }
        }

        true
    }
}
```
