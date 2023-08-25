# 926. 将字符串翻转到单调递增
如果一个二进制字符串，是以一些 `0`（可能没有 `0`）后面跟着一些 `1`（也可能没有 `1`）的形式组成的，那么该字符串是 **单调递增** 的。

给你一个二进制字符串 `s`，你可以将任何 `0` 翻转为 `1` 或者将 `1` 翻转为 `0` 。

返回使 `s` 单调递增的最小翻转次数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "00110"
<strong>输出:</strong> 1
<strong>解释:</strong> 翻转最后一位得到 00111.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "010110"
<strong>输出:</strong> 2
<strong>解释:</strong> 翻转得到 011111，或者是 000111。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "00011000"
<strong>输出:</strong> 2
<strong>解释:</strong> 翻转得到 00000000。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s[i]` 为 `'0'` 或 `'1'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = s.iter().filter(|&&c| c == b'0').count() as i32;
        let mut ret = count;

        for i in 0..s.len() {
            count += (s[i] == b'1') as i32 - (s[i] == b'0') as i32;

            ret = ret.min(count);
        }

        ret
    }
}
```
