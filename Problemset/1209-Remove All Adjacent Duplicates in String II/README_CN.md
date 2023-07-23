# 1209. 删除字符串中的所有相邻重复项 II
给你一个字符串 `s`，`k` 倍重复项删除操作」将会从 `s` 中选择 `k` 个相邻且相等的字母，并删除它们，使被删去的字符串的左侧和右侧连在一起。

你需要对 `s` 重复进行无限次这样的删除操作，直到无法继续为止。

在执行完所有删除操作后，返回最终得到的字符串。

本题答案保证唯一。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abcd", k = 2
<strong>输出:</strong> "abcd"
<strong>解释:</strong> 没有要删除的内容。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "deeedbbcccbdaa", k = 3
<strong>输出:</strong> "aa"
<strong>解释:</strong>
先删除 "eee" 和 "ccc"，得到 "ddbbbdaa"
再删除 "bbb"，得到 "dddaa"
最后删除 "ddd"，得到 "aa"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "pbbcggttciiippooaais", k = 2
<strong>输出:</strong> "ps"
</pre>

#### 提示:
* `1 <= s.length <= 10^5`
* `2 <= k <= 10^4`
* `s` 中只含有小写英文字母。

## 题解 (Rust)

### 1. 栈
```Rust
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(u8, i32)> = vec![];

        for ch in s.bytes() {
            match stack.last_mut() {
                Some(last) if ch == last.0 => {
                    last.1 += 1;
                    if last.1 == k {
                        stack.pop();
                    }
                }
                _ => stack.push((ch, 1)),
            }
        }

        String::from_utf8(
            stack
                .into_iter()
                .map(|(ch, cnt)| vec![ch; cnt as usize])
                .flatten()
                .collect(),
        )
        .unwrap()
    }
}
```
