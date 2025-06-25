# 2223. 构造字符串的总得分和
你需要从空字符串开始 **构造** 一个长度为 `n` 的字符串 `s` ，构造的过程为每次给当前字符串 **前面** 添加 **一个** 字符。构造过程中得到的所有字符串编号为 `1` 到 `n` ，其中长度为 `i` 的字符串编号为 <code>s<sub>i</sub></code> 。

* 比方说，`s = "abaca"` ，<code>s<sub>1</sub> == "a"</code> ，<code>s<sub>2</sub> == "ca"</code> ，<code>s<sub>3</sub> == "aca"</code> 依次类推。

<code>s<sub>i</sub></code> 的 **得分** 为 <code>s<sub>i</sub></code> 和 <code>s<sub>n</sub></code> 的 **最长公共前缀** 的长度（注意 <code>s == s<sub>n</sub></code> ）。

给你最终的字符串 `s` ，请你返回每一个 <code>s<sub>i</sub></code> 的 **得分之和** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "babab"
<strong>输出:</strong> 9
<strong>解释:</strong>
s1 == "b" ，最长公共前缀是 "b" ，得分为 1 。
s2 == "ab" ，没有公共前缀，得分为 0 。
s3 == "bab" ，最长公共前缀为 "bab" ，得分为 3 。
s4 == "abab" ，没有公共前缀，得分为 0 。
s5 == "babab" ，最长公共前缀为 "babab" ，得分为 5 。
得分和为 1 + 0 + 3 + 0 + 5 = 9 ，所以我们返回 9 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "azbazbzaz"
<strong>输出:</strong> 14
<strong>解释:</strong>
s2 == "az" ，最长公共前缀为 "az" ，得分为 2 。
s6 == "azbzaz" ，最长公共前缀为 "azb" ，得分为 3 。
s9 == "azbazbzaz" ，最长公共前缀为 "azbazbzaz" ，得分为 9 。
其他 si 得分均为 0 。
得分和为 2 + 3 + 9 = 14 ，所以我们返回 14 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_scores(s: String) -> i64 {
        const BASE: i64 = 131;
        const MOD: i64 = 1_000_000_007;
        let s = s.as_bytes();
        let mut prefix_pow = vec![1; s.len()];
        let mut prefix_hash = vec![s[0] as i64; s.len()];
        let mut ret = s.len();

        for i in 1..s.len() {
            prefix_pow[i] = prefix_pow[i - 1] * BASE % MOD;
            prefix_hash[i] = (prefix_hash[i - 1] * BASE + s[i] as i64) % MOD;
        }

        for i in 1..s.len() {
            if s[i] != s[0] {
                continue;
            }

            let mut l = 1;
            let mut r = s.len() - i + 1;

            while l < r {
                let m = (l + r) / 2;
                let hash =
                    (prefix_hash[i + m - 1] - prefix_hash[i - 1] * prefix_pow[m]).rem_euclid(MOD);

                if hash == prefix_hash[m - 1] {
                    l = m + 1;
                } else {
                    r = m;
                }
            }

            ret += l - 1;
        }

        ret as i64
    }
}
```
