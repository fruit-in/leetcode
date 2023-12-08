# 1016. 子串能表示从 1 到 N 数字的二进制串
给定一个二进制字符串 `s` 和一个正整数 `n`，如果对于 `[1, n]` 范围内的每个整数，*其二进制表示都是 `s` 的 **子字符串** ，就返回 `true`，否则返回 `false` *。

**子字符串** 是字符串中连续的字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "0110", n = 3
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "0110", n = 4
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s[i]` 不是 `'0'` 就是 `'1'`
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for x in 1..=n {
            if !s.contains(&format!("{:b}", x)) {
                return false;
            }
        }

        true
    }
}
```
