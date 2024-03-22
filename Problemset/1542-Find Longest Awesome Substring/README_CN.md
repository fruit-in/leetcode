# 1542. 找出最长的超赞子字符串
给你一个字符串 `s` 。请返回 `s` 中最长的 **超赞子字符串** 的长度。

「超赞子字符串」需满足满足下述两个条件：

* 该字符串是 `s` 的一个非空子字符串
* 进行任意次数的字符交换后，该字符串可以变成一个回文字符串

#### 示例 1:
<pre>
<strong>输入:</strong> s = "3242415"
<strong>输出:</strong> 5
<strong>解释:</strong> "24241" 是最长的超赞子字符串，交换其中的字符后，可以得到回文 "24142"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "12345678"
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "213123"
<strong>输出:</strong> 6
<strong>解释:</strong> "213123" 是最长的超赞子字符串，交换其中的字符后，可以得到回文 "231132"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "00"
<strong>输出:</strong> 2
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 仅由数字组成

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut hash = HashMap::from([(0, -1)]);
        let mut x = 0;
        let mut ret = 1;

        for (i, digit) in s.bytes().enumerate() {
            x ^= 1 << (digit - b'0');
            ret = ret.max(i as i32 - hash.get(&x).unwrap_or(&(i as i32)));
            for digit in 0..10 {
                ret = ret.max(i as i32 - hash.get(&(x ^ (1 << digit))).unwrap_or(&(i as i32)));
            }
            if !hash.contains_key(&x) {
                hash.insert(x, i as i32);
            }
        }

        ret
    }
}
```
