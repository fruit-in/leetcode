# 552. 学生出勤记录 II
可以用字符串表示一个学生的出勤记录，其中的每个字符用来标记当天的出勤情况（缺勤、迟到、到场）。记录中只含下面三种字符：

* `'A'`：Absent，缺勤
* `'L'`：Late，迟到
* `'P'`：Present，到场

如果学生能够 **同时** 满足下面两个条件，则可以获得出勤奖励：

* 按 **总出勤** 计，学生缺勤（`'A'`）**严格** 少于两天。
* 学生 **不会** 存在 **连续** 3 天或 **连续** 3 天以上的迟到（`'L'`）记录。

给你一个整数 `n` ，表示出勤记录的长度（次数）。请你返回记录长度为 `n` 时，可能获得出勤奖励的记录情况 **数量** 。答案可能很大，所以返回对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 8
<strong>解释:</strong>
有 8 种长度为 2 的记录将被视为可奖励：
"PP" , "AP", "PA", "LP", "PL", "AL", "LA", "LL"
只有"AA"不会被视为可奖励，因为缺勤次数为 2 次（需要少于 2 次）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 10101
<strong>输出:</strong> 183236316
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![[[0_i64; 3]; 2]; n + 1];
        let mut ret = 0;
        dp[0][0][0] = 1;

        for i in 0..n {
            dp[i + 1][1][0] = dp[i][0][0] + dp[i][0][1] + dp[i][0][2];
            for j in 0..2 {
                dp[i + 1][j][0] += dp[i][j][0] + dp[i][j][1] + dp[i][j][2];
                dp[i + 1][j][0] %= 1_000_000_007;
                for k in 0..2 {
                    dp[i + 1][j][k + 1] += dp[i][j][k];
                    dp[i + 1][j][k + 1] %= 1_000_000_007;
                }
            }
        }

        for j in 0..2 {
            for k in 0..3 {
                ret = (ret + dp[n][j][k] as i32) % 1_000_000_007;
            }
        }

        ret
    }
}
```
