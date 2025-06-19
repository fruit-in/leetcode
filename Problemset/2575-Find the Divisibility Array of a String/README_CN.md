# 2575. 找出字符串的可整除数组
给你一个下标从 **0** 开始的字符串 `word` ，长度为 `n` ，由从 `0` 到 `9` 的数字组成。另给你一个正整数 `m` 。

`word` 的 **可整除数组** `div`  是一个长度为 `n` 的整数数组，并满足：
* 如果 `word[0,...,i]` 所表示的 **数值** 能被 `m` 整除，`div[i] = 1`
* 否则，`div[i] = 0`

返回 `word` 的可整除数组。

#### 示例 1:
<pre>
<strong>输入:</strong> word = "998244353", m = 3
<strong>输出:</strong> [1,1,0,0,0,1,1,0,0]
<strong>解释:</strong> 仅有 4 个前缀可以被 3 整除："9"、"99"、"998244" 和 "9982443" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word = "1010", m = 10
<strong>输出:</strong> [0,1,0,1]
<strong>解释:</strong> 仅有 2 个前缀可以被 10 整除："10" 和 "1010" 。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* `word.length == n`
* `word` 由数字 `0` 到 `9` 组成
* <code>1 <= m <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let m = m as i64;
        let mut x = 0;
        let mut div = vec![0; word.len()];

        for (i, digit) in word.bytes().enumerate() {
            x = (x * 10 + (digit - b'0') as i64) % m;
            div[i] = 0.max(1 - x as i32);
        }

        div
    }
}
```
