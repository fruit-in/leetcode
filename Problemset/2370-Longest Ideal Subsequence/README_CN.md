# 2370. 最长理想子序列
给你一个由小写字母组成的字符串 `s` ，和一个整数 `k` 。如果满足下述条件，则可以将字符串 `t` 视作是 **理想字符串** ：
* `t` 是字符串 `s` 的一个子序列。
* `t` 中每两个 **相邻** 字母在字母表中位次的绝对差值小于或等于 `k` 。

返回 **最长** 理想字符串的长度。

字符串的子序列同样是一个字符串，并且子序列还满足：可以经由其他字符串删除某些字符（也可以不删除）但不改变剩余字符的顺序得到。

**注意：**字母表顺序不会循环。例如，`'a'` 和 `'z'` 在字母表中位次的绝对差值是 `25` ，而不是 `1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "acfgbd", k = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 最长理想字符串是 "acbd" 。该字符串长度为 4 ，所以返回 4 。
注意 "acfgbd" 不是理想字符串，因为 'c' 和 'f' 的字母表位次差值为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcd", k = 3
<strong>输出:</strong> 4
<strong>解释:</strong> 最长理想字符串是 "abcd" ，该字符串长度为 4 ，所以返回 4 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `0 <= k <= 25`
* `s` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let k = k as u8;
        let mut dp = [0; 26];

        for c in s.bytes() {
            dp[(c - b'a') as usize] = (b'a'.max(c - k)..=b'z'.min(c + k))
                .map(|i| dp[(i - b'a') as usize])
                .max()
                .unwrap()
                + 1;
        }

        *dp.iter().max().unwrap()
    }
}
```
