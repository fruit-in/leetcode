# 409. 最长回文串
给定一个包含大写字母和小写字母的字符串，找到通过这些字母构造成的最长的回文串。

在构造过程中，请注意区分大小写。比如 ```"Aa"``` 不能当做一个回文字符串。

#### 注意:
假设字符串的长度不会超过 1010。

#### 示例 1:
```
输入:
"abccccdd"

输出:
7

解释:
我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
```

## 题解 (Rust)

### 1. 贪心
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut count = HashMap::new();
        let mut result = 0;
        for ch in s.chars() {
            match count.get(&ch) {
                Some(i) => count.insert(ch, i + 1),
                None => count.insert(ch, 1),
            };
        }
        for v in count.values() {
            if v % 2 == 0 || result % 2 == 0 {
                result += v;
            } else {
                result += v - 1;
            }
        }
        result
    }
}
```
