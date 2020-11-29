# 1525. 字符串的好分割数目
给你一个字符串 `s` ，一个分割被称为 「好分割」 当它满足：将 `s` 分割成 2 个字符串 `p` 和 `q` ，它们连接起来等于 `s` 且 `p` 和 `q` 中不同字符的数目相同。

请你返回 `s` 中好分割的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aacaba"
<strong>输出:</strong> 2
<strong>解释:</strong> 总共有 5 种分割字符串 "aacaba" 的方法，其中 2 种是好分割。
("a", "acaba") 左边字符串和右边字符串分别包含 1 个和 3 个不同的字符。
("aa", "caba") 左边字符串和右边字符串分别包含 1 个和 3 个不同的字符。
("aac", "aba") 左边字符串和右边字符串分别包含 2 个和 2 个不同的字符。这是一个好分割。
("aaca", "ba") 左边字符串和右边字符串分别包含 2 个和 2 个不同的字符。这是一个好分割。
("aacab", "a") 左边字符串和右边字符串分别包含 3 个和 1 个不同的字符。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcd"
<strong>输出:</strong> 1
<strong>解释:</strong> 好分割为将字符串分割成 ("ab", "cd") 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aaaaa"
<strong>输出:</strong> 4
<strong>解释:</strong> 所有分割都是好分割。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "acbadbaada"
<strong>输出:</strong> 2
</pre>

#### 提示:
* `s` 只包含小写英文字母。
* `1 <= s.length <= 10^5`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut p_count = vec![];
        let mut q_count = vec![];
        let mut count = 0i32;

        for ch in s.bytes().take(s.len() - 1) {
            count |= 1 << (ch - b'a');
            p_count.push(count.count_ones());
        }
        count = 0;
        for ch in s.bytes().rev().take(s.len() - 1) {
            count |= 1 << (ch - b'a');
            q_count.push(count.count_ones());
        }

        p_count
            .iter()
            .zip(q_count.iter().rev())
            .filter(|(x, y)| x == y)
            .count() as i32
    }
}
```
