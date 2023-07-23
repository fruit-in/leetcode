# 242. Valid Anagram
Given two strings *s* and *t* , write a function to determine if *t* is an anagram of *s*.

#### Example 1:
<pre>
<strong>Input:</strong> <em>s</em> = "anagram", <em>t</em> = "nagaram"
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> <em>s</em> = "rat", <em>t</em> = "car"
<strong>Output:</strong> false
</pre>

#### Note:
You may assume the string contains only lowercase alphabets.

#### Follow up:
What if the inputs contain unicode characters? How would you adapt your solution to such case?

## Solutions (Rust)

### 1. Sort
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

### 2. HashMap
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
