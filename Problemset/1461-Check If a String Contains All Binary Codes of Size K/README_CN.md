# 1461. 检查一个字符串是否包含所有长度为 K 的二进制子串
给你一个二进制字符串 `s` 和一个整数 `k` 。

如果所有长度为 `k` 的二进制字符串都是 `s` 的子串，请返回 True ，否则请返回 False 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "00110110", k = 2
<strong>输出:</strong> true
<strong>解释:</strong> 长度为 2 的二进制串包括 "00"，"01"，"10" 和 "11"。它们分别是 s 中下标为 0，1，3，2 开始的长度为 2 的子串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "00110", k = 2
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "0110", k = 1
<strong>输出:</strong> true
<strong>解释:</strong> 长度为 1 的二进制串包括 "0" 和 "1"，显然它们都是 s 的子串。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "0110", k = 2
<strong>输出:</strong> false
<strong>解释:</strong> 长度为 2 的二进制串 "00" 没有出现在 s 中。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = "0000000001011100", k = 4
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= s.length <= 5 * 10^5`
* `s` 中只含 0 和 1 。
* `1 <= k <= 20`

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }

        let mut x = i32::from_str_radix(&s[..k as usize], 2).unwrap();
        let y = 1 << k;
        let mut set = vec![x].into_iter().collect::<HashSet<_>>();

        for c in s.bytes().skip(k as usize) {
            x = (x << 1) % y + (c - b'0') as i32;
            set.insert(x);
        }

        set.len() == y as usize
    }
}
```
