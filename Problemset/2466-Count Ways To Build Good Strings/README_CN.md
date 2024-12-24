# 2466. 统计构造好字符串的方案数
给你整数 `zero` ，`one` ，`low` 和 `high` ，我们从空字符串开始构造一个字符串，每一步执行下面操作中的一种：

* 将 `'0'` 在字符串末尾添加 `zero`  次。
* 将 `'1'` 在字符串末尾添加 `one` 次。

以上操作可以执行任意次。

如果通过以上过程得到一个 **长度** 在 `low` 和 `high` 之间（包含上下边界）的字符串，那么这个字符串我们称为 **好** 字符串。

请你返回满足以上要求的 **不同** 好字符串数目。由于答案可能很大，请将结果对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> low = 3, high = 3, zero = 1, one = 1
<strong>输出:</strong> 8
<strong>解释:</strong>
一个可能的好字符串是 "011" 。
可以这样构造得到："" -> "0" -> "01" -> "011" 。
从 "000" 到 "111" 之间所有的二进制字符串都是好字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> low = 2, high = 3, zero = 1, one = 2
<strong>输出:</strong> 5
<strong>解释:</strong> 好字符串为 "00" ，"11" ，"000" ，"110" 和 "011" 。
</pre>

#### 提示:
* <code>1 <= low <= high <= 10<sup>5</sup></code>
* `1 <= zero, one <= low`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let (low, high) = (low as usize, high as usize);
        let (zero, one) = (zero as usize, one as usize);
        let mut dp = vec![0; high + 1];
        let mut ret = 0;
        dp[0] = 1;

        for i in 0..=high {
            if i >= low {
                ret = (ret + dp[i]) % 1_000_000_007;
            }

            if i + zero <= high {
                dp[i + zero] = (dp[i + zero] + dp[i]) % 1_000_000_007;
            }
            if i + one <= high {
                dp[i + one] = (dp[i + one] + dp[i]) % 1_000_000_007;
            }
        }

        ret
    }
}
```
