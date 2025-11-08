# 1987. 不同的好子序列数目
给你一个二进制字符串 `binary` 。 `binary` 的一个 **子序列** 如果是 **非空** 的且没有 **前导 0** （除非数字是 `"0"` 本身），那么它就是一个 **好** 的子序列。

请你找到 `binary` **不同好子序列** 的数目。

* 比方说，如果 `binary = "001"` ，那么所有 **好** 子序列为 `["0", "0", "1"]` ，所以 **不同** 的好子序列为 `"0"` 和 `"1"` 。 注意，子序列 `"00"` ，`"01"` 和 `"001"` 不是好的，因为它们有前导 0 。

请你返回 `binary` 中 **不同好子序列** 的数目。由于答案可能很大，请将它对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

一个 **子序列** 指的是从原数组中删除若干个（可以一个也不删除）元素后，不改变剩余元素顺序得到的序列。

#### 示例 1:
<pre>
<strong>输入:</strong> binary = "001"
<strong>输出:</strong> 2
<strong>解释:</strong> 好的二进制子序列为 ["0", "0", "1"] 。
不同的好子序列为 "0" 和 "1" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> binary = "11"
<strong>输出:</strong> 2
<strong>解释:</strong> 好的二进制子序列为 ["1", "1", "11"] 。
不同的好子序列为 "1" 和 "11" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> binary = "101"
<strong>输出:</strong> 5
<strong>解释:</strong> 好的二进制子序列为 ["1", "0", "1", "10", "11", "101"] 。
不同的好子序列为 "0" ，"1" ，"10" ，"11" 和 "101" 。
</pre>

#### 提示:
* <code>1 <= binary.length <= 10<sup>5</sup></code>
* `binary` 只含有 `'0'` 和 `'1'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = binary.len();
        let mut last_index = [n; 2];
        let mut first0 = false;
        let mut next = [false; 2];
        let mut dp = vec![0_i32; n + 1];
        dp[n] = -1;

        for (i, b) in binary.bytes().map(|b| (b - b'0') as usize).enumerate() {
            dp[i + 1] = (dp[i] * 2 - dp[last_index[b]]).rem_euclid(MOD);
            last_index[b] = i;

            if next[b] {
                dp[i + 1] = (dp[i + 1] - 1).rem_euclid(MOD);
                next[b] = false;
            }
            if b == 0 && !first0 {
                first0 = true;
                next = [true; 2];
            }
        }

        *dp.last().unwrap()
    }
}
```
