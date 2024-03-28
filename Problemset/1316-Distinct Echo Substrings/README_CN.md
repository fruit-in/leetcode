# 1316. 不同的循环子字符串
给你一个字符串 `text` ，请你返回满足下述条件的 **不同** 非空子字符串的数目：

* 可以写成某个字符串与其自身相连接的形式（即，可以写为 `a + a`，其中 `a` 是某个字符串）。

例如，`abcabc` 就是 `abc` 和它自身连接形成的。

#### 示例 1:
<pre>
<strong>输入:</strong> text = "abcabcabc"
<strong>输出:</strong> 3
<strong>解释:</strong> 3 个子字符串分别为 "abcabc"，"bcabca" 和 "cabcab" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> text = "leetcodeleetcode"
<strong>输出:</strong> 2
<strong>解释:</strong> 2 个子字符串为 "ee" 和 "leetcodeleetcode" 。
</pre>

#### 提示:
* `1 <= text.length <= 2000`
* `text` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let bytes = text.as_bytes();
        let mut subends: HashMap<&str, HashSet<usize>> = HashMap::new();
        let mut halfsubs = HashSet::new();

        for i in 0..text.len() {
            for j in i..=((text.len() + i - 2) / 2)
                .max((2 * i).saturating_sub(1))
                .min(text.len() - 1)
            {
                if i > 0
                    && j < text.len() - 1
                    && bytes[i - 1] != bytes[j]
                    && bytes[i] != bytes[j + 1]
                {
                    continue;
                }

                let s = text.get(i..=j).unwrap();

                if halfsubs.contains(&s) {
                    continue;
                }

                if i > 0
                    && j <= 2 * i - 1
                    && subends
                        .get(&s)
                        .unwrap_or(&HashSet::new())
                        .contains(&(i - 1))
                {
                    halfsubs.insert(s);
                } else if 2 * j < text.len() - 1 + i {
                    subends.entry(s).or_insert(HashSet::new()).insert(j);
                }
            }
        }

        halfsubs.len() as i32
    }
}
```
