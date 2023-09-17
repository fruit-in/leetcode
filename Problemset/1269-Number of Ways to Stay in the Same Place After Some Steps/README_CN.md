# 1269. 停在原地的方案数
有一个长度为 `arrLen` 的数组，开始有一个指针在索引 `0` 处。

每一步操作中，你可以将指针向左或向右移动 1 步，或者停在原地（指针不能被移动到数组范围外）。

给你两个整数 `steps` 和 `arrLen` ，请你计算并返回：在恰好执行 `steps` 次操作以后，指针仍然指向索引 `0` 处的方案数。

由于答案可能会很大，请返回方案数 **模** `10^9 + 7` 后的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> steps = 3, arrLen = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 3 步后，总共有 4 种不同的方法可以停在索引 0 处。
向右，向左，不动
不动，向右，向左
向右，不动，向左
不动，不动，不动
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> steps = 2, arrLen = 4
<strong>输出:</strong> 2
<strong>解释:</strong> 2 步后，总共有 2 种不同的方法可以停在索引 0 处。
向右，向左
不动，不动
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> steps = 4, arrLen = 2
<strong>输出:</strong> 8
</pre>

#### 提示:
* `1 <= steps <= 500`
* <code>1 <= arrLen <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let steps = steps as usize;
        let arr_len = arr_len as usize;
        let mut dp = vec![vec![0; arr_len.min(steps / 2 + 1)]; steps + 1];
        dp[0][0] = 1;

        for i in 0..steps {
            for j in 0..dp[0].len() {
                dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % 1_000_000_007;
                if j > 0 {
                    dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % 1_000_000_007;
                }
                if j < dp[0].len() - 1 {
                    dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % 1_000_000_007;
                }
            }
        }

        dp[steps][0]
    }
}
```
