# 389. 找不同
给定两个字符串 ***s*** 和 ***t***，它们只包含小写字母。

字符串 ***t*** 由字符串 ***s*** 随机重排，然后在随机位置添加一个字母。

请找出在 ***t*** 中被添加的字母。

#### 示例:
```
输入:
s = "abcd"
t = "abcde"

输出:
e

解释:
'e' 是那个被添加的字母。
```

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut cnt = [0; 26];

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        for ch in t.bytes() {
            cnt[(ch - b'a') as usize] -= 1;
            if cnt[(ch - b'a') as usize] == -1 {
                return ch as char;
            }
        }

        ' '
    }
}
```

### 2. 异或
```Rust
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ret = 0;

        for ch in s.bytes() {
            ret ^= ch;
        }

        for ch in t.bytes() {
            ret ^= ch;
        }

        ret as char
    }
}
```
