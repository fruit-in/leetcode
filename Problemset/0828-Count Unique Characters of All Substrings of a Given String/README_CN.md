# 828. 统计子串中的唯一字符
我们定义了一个函数 `countUniqueChars(s)` 来统计字符串 `s` 中的唯一字符，并返回唯一字符的个数。

例如：`s = "LEETCODE"` ，则其中 `"L"`, `"T"`,`"C"`,`"O"`,`"D"` 都是唯一字符，因为它们只出现一次，所以 `countUniqueChars(s) = 5` 。

本题将会给你一个字符串 `s` ，我们需要返回 `countUniqueChars(t)` 的总和，其中 `t` 是 `s` 的子字符串。输入用例保证返回值为 32 位整数。

注意，某些子字符串可能是重复的，但你统计时也必须算上这些重复的子字符串（也就是说，你必须统计 `s` 的所有子字符串中的唯一字符）。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "ABC"
<strong>输出:</strong> 10
<strong>解释:</strong> 所有可能的子串为："A","B","C","AB","BC" 和 "ABC"。
     其中，每一个子串都由独特字符构成。
     所以其长度总和为：1 + 1 + 1 + 2 + 2 + 3 = 10
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "ABA"
<strong>输出:</strong> 8
<strong>解释:</strong> 除了 countUniqueChars("ABA") = 1 之外，其余与示例 1 相同。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "LEETCODE"
<strong>输出:</strong> 92
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含大写英文字符

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut indices = vec![vec![-1]; 26];
        let mut ret = 0;

        for i in 0..s.len() {
            indices[(s[i] - b'A') as usize].push(i as i32);
        }

        for i in 0..26 {
            indices[i].push(s.len() as i32);

            for j in 1..indices[i].len() - 1 {
                ret += (indices[i][j] - indices[i][j - 1]) * (indices[i][j + 1] - indices[i][j]);
            }
        }

        ret
    }
}
```
