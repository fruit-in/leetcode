# 2484. 统计回文子序列数目
给你数字字符串 `s` ，请你返回 `s` 中长度为 `5` 的 **回文子序列** 数目。由于答案可能很大，请你将答案对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 提示：
* 如果一个字符串从前往后和从后往前读相同，那么它是 **回文字符串** 。
* 子序列是一个字符串中删除若干个字符后，不改变字符顺序，剩余字符构成的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "103301"
<strong>输出:</strong> 2
<strong>解释:</strong>
总共有 6 长度为 5 的子序列："10330" ，"10331" ，"10301" ，"10301" ，"13301" ，"03301" 。
它们中有两个（都是 "10301"）是回文的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "0000000"
<strong>输出:</strong> 21
<strong>解释:</strong> 所有 21 个长度为 5 的子序列都是 "00000" ，都是回文的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "9999900000"
<strong>输出:</strong> 2
<strong>解释:</strong> 仅有的两个回文子序列是 "99999" 和 "00000" 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>4</sup></code>
* `s` 只包含数字字符。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_palindromes(s: String) -> i32 {
        let s = s.bytes().map(|c| (c - b'0') as usize).collect::<Vec<_>>();
        let mut count1 = [0_i64; 10];
        let mut count2l = vec![[0; 100]; s.len()];
        let mut count2r = vec![[0; 100]; s.len()];
        let mut ret = 0;

        count1[s[0]] = 1;
        for i in 1..s.len() {
            count2l[i] = count2l[i - 1];
            for j in 0..10 {
                count2l[i][s[i] * 10 + j] = (count2l[i][s[i] * 10 + j] + count1[j]) % 1_000_000_007;
            }
            count1[s[i]] += 1;
        }

        count1 = [0; 10];
        count1[s[s.len() - 1]] = 1;
        for i in (0..s.len() - 1).rev() {
            count2r[i] = count2r[i + 1];
            for j in 0..10 {
                count2r[i][s[i] * 10 + j] = (count2r[i][s[i] * 10 + j] + count1[j]) % 1_000_000_007;
            }
            count1[s[i]] += 1;
        }

        for i in 2..s.len().saturating_sub(2) {
            for j in 0..100 {
                ret = (ret + count2l[i - 1][j] * count2r[i + 1][j] % 1_000_000_007) % 1_000_000_007;
            }
        }

        ret as i32
    }
}
```
