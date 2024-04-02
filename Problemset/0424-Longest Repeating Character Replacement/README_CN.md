# 424. 替换后的最长重复字符
给你一个字符串 `s` 和一个整数 `k` 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 `k` 次。

在执行上述操作后，返回 *包含相同字母的最长子字符串的长度*。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "ABAB", k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 用两个'A'替换为两个'B',反之亦然。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "AABABBA", k = 1
<strong>输出:</strong> 4
<strong>解释:</strong>
将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
子串 "BBBB" 有最长重复字母, 答案为 4。
可能存在其他的方法来得到同样的结果。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 仅由大写英文字母组成
* `0 <= k <= s.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 26];
        let mut most_ch = b'A';
        let mut i = -1;
        let mut ret = 0;

        for j in 0..s.len() {
            count[(s[j] - b'A') as usize] += 1;
            if count[(s[j] - b'A') as usize] > count[(most_ch - b'A') as usize] {
                most_ch = s[j];
            }
            while j as i32 - i - count[(most_ch - b'A') as usize] > k {
                i += 1;
                count[(s[i as usize] - b'A') as usize] -= 1;
                if s[i as usize] == most_ch {
                    most_ch = (0..26).max_by_key(|&i| count[i]).unwrap() as u8 + b'A';
                }
            }

            ret = ret.max(j as i32 - i);
        }

        ret
    }
}
```
