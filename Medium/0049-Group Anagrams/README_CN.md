# 49. 字母异位词分组
给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。

#### 示例:
<pre>
<strong>输入:</strong> ["eat", "tea", "tan", "ate", "nat", "bat"]
<strong>输出:</strong>
[
  ["ate","eat","tea"],
  ["nat","tan"],
  ["bat"]
]
</pre>

#### 说明:
* 所有输入均为小写字母。
* 不考虑答案输出的顺序。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();

        for s in strs {
            let mut cnt = [0; 26];
            s.bytes().for_each(|c| cnt[(c - b'a') as usize] += 1);
            anagrams.entry(cnt).or_insert(Vec::new()).push(s);
        }

        anagrams.values().cloned().collect()
    }
}
```
