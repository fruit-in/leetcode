# 1420. 生成数组
给你三个整数 `n`、`m` 和 `k` 。下图描述的算法用于找出正整数数组中最大的元素。

![](https://assets.leetcode.com/uploads/2020/04/02/e.png)

请你生成一个具有下述属性的数组 `arr` ：

* `arr` 中有 `n` 个整数。
* `1 <= arr[i] <= m` 其中 `(0 <= i < n)` 。
* 将上面提到的算法应用于 `arr` ，`search_cost` 的值等于 `k` 。

返回上述条件下生成数组 `arr` 的 **方法数** ，由于答案可能会很大，所以 **必须** 对 `10^9 + 7` 取余。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2, m = 3, k = 1
<strong>输出:</strong> 6
<strong>解释:</strong> 可能的数组分别为 [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 5, m = 2, k = 3
<strong>输出:</strong> 0
<strong>解释:</strong> 没有数组可以满足上述条件
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 9, m = 1, k = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 可能的数组只有 [1, 1, 1, 1, 1, 1, 1, 1, 1]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 50, m = 100, k = 25
<strong>输出:</strong> 34549172
<strong>解释:</strong> 不要忘了对 1000000007 取余
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> n = 37, m = 17, k = 7
<strong>输出:</strong> 418930126
</pre>

#### 提示:
* `1 <= n <= 50`
* `1 <= m <= 100`
* `0 <= k <= n`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = vec![vec![vec![0; m as usize + 1]; k as usize + 1]; n as usize + 1];
        dp[0][0][0] = 1;

        for a in 0..dp.len() - 1 {
            for b in 0..dp[0].len() {
                for c in 0..dp[0][0].len() {
                    dp[a + 1][b][c] = (dp[a + 1][b][c] + dp[a][b][c] * c as i64) % MOD;
                    if b < dp[0].len() - 1 {
                        for d in c + 1..dp[0][0].len() {
                            dp[a + 1][b + 1][d] = (dp[a + 1][b + 1][d] + dp[a][b][c]) % MOD;
                        }
                    }
                }
            }
        }

        dp[n as usize][k as usize]
            .iter()
            .fold(0, |acc, x| (acc + x) % MOD) as i32
    }
}
```
