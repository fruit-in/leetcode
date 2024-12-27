# 2516. 每种字符至少取 K 个
给你一个由字符 `'a'`、`'b'`、`'c'` 组成的字符串 `s` 和一个非负整数 `k` 。每分钟，你可以选择取走 `s` **最左侧** 还是 **最右侧** 的那个字符。

你必须取走每种字符 **至少** `k` 个，返回需要的 **最少** 分钟数；如果无法取到，则返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aabaaaacaabc", k = 2
<strong>输出:</strong> 8
<strong>解释:</strong>
从 s 的左侧取三个字符，现在共取到两个字符 'a' 、一个字符 'b' 。
从 s 的右侧取五个字符，现在共取到四个字符 'a' 、两个字符 'b' 和两个字符 'c' 。
共需要 3 + 5 = 8 分钟。
可以证明需要的最少分钟数是 8 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "a", k = 1
<strong>输出:</strong> -1
<strong>解释:</strong> 无法取到一个字符 'b' 或者 'c'，所以返回 -1 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 仅由字母 `'a'`、`'b'`、`'c'` 组成
* `0 <= k <= s.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count_a = 0;
        let mut count_b = 0;
        let mut count_c = 0;
        let mut i = 0;
        let mut ret: usize;

        while i < n && (count_a < k || count_b < k || count_c < k) {
            count_a += (s[i] == b'a') as i32;
            count_b += (s[i] == b'b') as i32;
            count_c += (s[i] == b'c') as i32;
            i += 1;
        }

        if count_a < k || count_b < k || count_c < k {
            return -1;
        }

        ret = i;

        for j in 1..n {
            count_a += (s[n - j] == b'a') as i32;
            count_b += (s[n - j] == b'b') as i32;
            count_c += (s[n - j] == b'c') as i32;

            while i > 0
                && count_a - (s[i - 1] == b'a') as i32 >= k
                && count_b - (s[i - 1] == b'b') as i32 >= k
                && count_c - (s[i - 1] == b'c') as i32 >= k
            {
                count_a -= (s[i - 1] == b'a') as i32;
                count_b -= (s[i - 1] == b'b') as i32;
                count_c -= (s[i - 1] == b'c') as i32;
                i -= 1;
            }

            if count_a >= k && count_b >= k && count_c >= k {
                ret = ret.min(i + j);
            }
        }

        ret as i32
    }
}
```
