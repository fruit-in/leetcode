# 451. 根据字符出现频率排序
给定一个字符串，请将字符串里的字符按照出现的频率降序排列。

#### 示例 1:
<pre>
<b>输入:</b> "tree"
<b>输出:</b> "eert"
<b>解释:</b> 'e'出现两次，'r'和't'都只出现一次。
因此'e'必须出现在'r'和't'之前。此外，"eetr"也是一个有效的答案。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> "cccaaa"
<b>输出:</b> "cccaaa"
<b>解释:</b> 'c'和'a'都出现三次。此外，"aaaccc"也是有效的答案。
注意"cacaca"是不正确的，因为相同的字母必须放在一起。
</pre>

#### 示例 3:
<pre>
<b>输入:</b> "Aabb"
<b>输出:</b> "bbAa"
<b>解释:</b> 此外，"bbaA"也是一个有效的答案，但"Aabb"是不正确的。
注意'A'和'a'被认为是两种不同的字符。
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut counter = HashMap::new();

        for ch in s.clone() {
            *counter.entry(ch).or_insert(0) -= 1;
        }

        s.sort_unstable();
        s.sort_by_key(|k| counter.get(&k));

        s.iter().collect()
    }
}
```
