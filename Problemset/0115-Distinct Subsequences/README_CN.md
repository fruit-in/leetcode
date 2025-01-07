# 115. 不同的子序列
给你两个字符串 `s` 和 `t` ，统计并返回在 `s` 的 **子序列** 中 `t` 出现的个数，结果需要对 10<sup>9</sup> + 7 取模。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "rabbbit", t = "rabbit"
<strong>输出:</strong> 3
<strong>解释:</strong>
如下所示, 有 3 种可以从 s 中得到 "rabbit" 的方案。
rabbbit
rabbbit
rabbbit
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "babgbag", t = "bag"
<strong>输出:</strong> 5
<strong>解释:</strong>
如下所示, 有 5 种可以从 s 中得到 "bag" 的方案。
babgbag
babgbag
babgbag
babgbag
babgbag
</pre>

#### 提示:
* `1 <= s.length, t.length <= 1000`
* `s` 和 `t` 由英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![0; t.len()]; s.len()];

        for i in 0..s.len() {
            for j in 0..t.len().min(i + 1) {
                if i > 0 {
                    dp[i][j] = dp[i - 1][j];
                }
                if s[i] == t[j] {
                    if j == 0 {
                        dp[i][j] += 1;
                    } else {
                        dp[i][j] = (dp[i][j] + dp[i - 1][j - 1]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[s.len() - 1][t.len() - 1]
    }
}
```
