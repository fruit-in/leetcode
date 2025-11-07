# 940. 不同的子序列 II
给定一个字符串 `s`，计算 `s` 的 **不同非空子序列** 的个数。因为结果可能很大，所以返回答案需要对 `10^9 + 7` **取余** 。

字符串的 **子序列** 是经由原字符串删除一些（也可能不删除）字符但不改变剩余字符相对位置的一个新字符串。

* 例如，`"ace"` 是 `"abcde"` 的一个子序列，但 `"aec"` 不是。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abc"
<strong>输出:</strong> 7
<strong>解释:</strong> 7 个不同的子序列分别是 "a", "b", "c", "ab", "ac", "bc", 以及 "abc"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "aba"
<strong>输出:</strong> 6
<strong>解释:</strong> 6 个不同的子序列分别是 "a", "b", "ab", "ba", "aa" 以及 "aba"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "aaa"
<strong>输出:</strong> 3
<strong>解释:</strong> 3 个不同的子序列分别是 "a", "aa" 以及 "aaa"。
</pre>

#### 提示:
* `1 <= s.length <= 2000`
* `s` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = s.len();
        let mut last_index = [n; 26];
        let mut dp = vec![0_i32; n + 1];
        dp[n] = -1;

        for (i, c) in s.bytes().map(|c| (c - b'a') as usize).enumerate() {
            dp[i + 1] = (dp[i] * 2 - dp[last_index[c]]).rem_euclid(MOD);
            last_index[c] = i;
        }

        *dp.last().unwrap()
    }
}
```
