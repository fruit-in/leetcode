# 1358. 包含所有三种字符的子字符串数目
给你一个字符串 `s` ，它只包含三种字符 a, b 和 c 。

请你返回 a，b 和 c 都 **至少** 出现过一次的子字符串数目。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abcabc"
<strong>输出:</strong> 10
<strong>解释:</strong> 包含 a，b 和 c 各至少一次的子字符串为 "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" 和 "abc" (相同字符串算多次)。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aaacb"
<strong>输出:</strong> 3
<strong>解释:</strong> 包含 a，b 和 c 各至少一次的子字符串为 "aaacb", "aacb" 和 "acb" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "abc"
<strong>输出:</strong> 1
</pre>

#### 提示:
* `3 <= s.length <= 5 x 10^4`
* `s` 只包含字符 a，b 和 c 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = [0; 3];
        let mut i = 0;
        let mut ret = 0;

        for j in 0..s.len() {
            count[(s[j] - b'a') as usize] += 1;

            while count[0] * count[1] * count[2] > 0 {
                count[(s[i] - b'a') as usize] -= 1;
                i += 1;
            }

            ret += i;
        }

        ret as i32
    }
}
```
