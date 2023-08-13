# 1420. Build Array Where You Can Find The Maximum Exactly K Comparisons
You are given three integers `n`, `m` and `k`. Consider the following algorithm to find the maximum element of an array of positive integers:

![](https://assets.leetcode.com/uploads/2020/04/02/e.png)

You should build the array arr which has the following properties:

* `arr` has exactly `n` integers.
* `1 <= arr[i] <= m` where `(0 <= i < n)`.
* After applying the mentioned algorithm to `arr`, the value `search_cost` is equal to `k`.

Return *the number of ways* to build the array `arr` under the mentioned conditions. As the answer may grow large, the answer **must be** computed modulo <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2, m = 3, k = 1
<strong>Output:</strong> 6
<strong>Explanation:</strong> The possible arrays are [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 5, m = 2, k = 3
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no possible arrays that satisify the mentioned conditions.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 9, m = 1, k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only possible array is [1, 1, 1, 1, 1, 1, 1, 1, 1]
</pre>

#### Constraints:
* `1 <= n <= 50`
* `1 <= m <= 100`
* `0 <= k <= n`

## Solutions (Rust)

### 1. Solution
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
