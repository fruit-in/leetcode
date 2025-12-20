# 730. 统计不同回文子序列
给你一个字符串 `s` ，返回 `s` 中不同的非空回文子序列个数 。由于答案可能很大，请返回对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

字符串的子序列可以经由字符串删除 0 个或多个字符获得。

如果一个序列与它反转后的序列一致，那么它是回文序列。

如果存在某个 `i` , 满足 <code>a<sub>i</sub> != b<sub>i</sub></code> ，则两个序列 <code>a<sub>1</sub>, a<sub>2</sub>, ...</code> 和 <code>b<sub>1</sub>, b<sub>2</sub>, ...</code> 不同。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "bccb"
<strong>输出:</strong> 6
<strong>解释:</strong> 6 个不同的非空回文子字符序列分别为：'b', 'c', 'bb', 'cc', 'bcb', 'bccb'。
注意：'bcb' 虽然出现两次但仅计数一次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba"
<strong>输出:</strong> 104860361
<strong>解释:</strong> 共有 3104860382 个不同的非空回文子序列，104860361 是对 10<sup>9</sup> + 7 取余后的值。
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s[i]` 仅包含 `'a'`, `'b'`, `'c'` 或 `'d'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0_i32; n]; n];

        for i in (0..n).rev() {
            let mut first = [n; 4];
            let mut last = [n; 4];

            for j in i..n {
                let k = (s[j] - b'a') as usize;
                dp[i][j] = dp[i][j.saturating_sub(1)];

                if first[k] == n {
                    dp[i][j] += 1;
                    first[k] = j;
                } else if first[k] == last[k] {
                    dp[i][j] += dp[first[k] + 1][j - 1] + 1;
                } else {
                    dp[i][j] += dp[first[k] + 1][j - 1] - dp[first[k] + 1][last[k] - 1];
                }

                dp[i][j] = dp[i][j].rem_euclid(1_000_000_007);
                last[k] = j;
            }
        }

        dp[0][n - 1]
    }
}
```
