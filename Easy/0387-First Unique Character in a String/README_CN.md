# 387. 字符串中的第一个唯一字符
给定一个字符串，找到它的第一个不重复的字符，并返回它的索引。如果不存在，则返回 -1。

#### 案例:
```
s = "leetcode"
返回 0.

s = "loveleetcode",
返回 2.
```

**注意事项:** 您可以假定该字符串只包含小写字母。

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnt = [0; 26];

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        for (i, ch) in s.bytes().enumerate() {
            if cnt[(ch - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}
```
