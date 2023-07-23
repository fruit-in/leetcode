# 242. 有效的字母异位词
给定两个字符串 *s* 和 *t* ，编写一个函数来判断 *t* 是否是 *s* 的字母异位词。

#### 示例 1:
<pre>
<strong>输入:</strong> <em>s</em> = "anagram", <em>t</em> = "nagaram"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> <em>s</em> = "rat", <em>t</em> = "car"
<strong>输出:</strong> false
</pre>

#### 说明:
你可以假设字符串只包含小写字母。

#### 进阶:
如果输入字符串包含 unicode 字符怎么办？你能否调整你的解法来应对这种情况？

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut s_vec: Vec<char> = s.chars().collect();
        let mut t_vec: Vec<char> = t.chars().collect();
        s_vec.sort_unstable();
        t_vec.sort_unstable();
        s_vec == t_vec
    }
}
```

### 2. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let s_vec: Vec<char> = s.chars().collect();
        let t_vec: Vec<char> = t.chars().collect();
        let mut map = HashMap::new();
        for i in 0..s_vec.len() {
            *map.entry(s_vec[i]).or_insert(0) += 1;
            *map.entry(t_vec[i]).or_insert(0) -= 1;
        }
        map.values().filter(|x| **x != 0).count() == 0
    }
}
```
