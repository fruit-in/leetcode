# 2309. 兼具大小写的最好英文字母
给你一个由英文字母组成的字符串 `s` ，请你找出并返回 `s` 中的 **最好** 英文字母。返回的字母必须为大写形式。如果不存在满足条件的字母，则返回一个空字符串。

**最好** 英文字母的大写和小写形式必须 **都** 在 `s` 中出现。

英文字母 `b` 比另一个英文字母 `a` **更好** 的前提是：英文字母表中，`b` 在 `a` 之 **后** 出现。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "lEeTcOdE"
<strong>输出:</strong> "E"
<strong>解释:</strong>
字母 'E' 是唯一一个大写和小写形式都出现的字母。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "arRAzFif"
<strong>输出:</strong> "R"
<strong>解释:</strong>
字母 'R' 是大写和小写形式都出现的最好英文字母。
注意 'A' 和 'F' 的大写和小写形式也都出现了，但是 'R' 比 'F' 和 'A' 更好。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "AbCdEfGhIjK"
<strong>输出:</strong> ""
<strong>解释:</strong>
不存在大写和小写形式都出现的字母。
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s` 由小写和大写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut occurs = [(false, false); 26];

        s.bytes().for_each(|c| match c {
            b'a'..=b'z' => occurs[(c - b'a') as usize].0 = true,
            _ => occurs[(c - b'A') as usize].1 = true,
        });

        match (0..26).rposition(|i| occurs[i] == (true, true)) {
            Some(i) => String::from_utf8(vec![i as u8 + b'A']).unwrap(),
            None => "".to_string(),
        }
    }
}
```
