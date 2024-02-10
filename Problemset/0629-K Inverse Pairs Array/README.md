# 629. K Inverse Pairs Array
For an integer array `nums`, an **inverse pair** is a pair of integers `[i, j]` where `0 <= i < j < nums.length` and `nums[i] > nums[j]`.

Given two integers n and k, return the number of different arrays consisting of numbers from `1` to `n` such that there are exactly `k` **inverse pairs**. Since the answer can be huge, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, k = 0
<strong>Output:</strong> 1
<strong>Explanation:</strong> Only the array [1,2,3] which consists of numbers from 1 to 3 has exactly 0 inverse pairs.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, k = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> The array [1,3,2] and [2,1,3] have exactly 1 inverse pair.
</pre>

#### Constraints:
* `1 <= n <= 1000`
* `0 <= k <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![0_i32; k + 1]; n + 1];

        dp[1][0] = 1;

        for i in 2..=n {
            for j in 0..=k {
                dp[i][j] = dp[i - 1][j];
                if j > 0 {
                    dp[i][j] = (dp[i][j] + dp[i][j - 1]) % 1_000_000_007;
                }
                if j >= i {
                    dp[i][j] = (dp[i][j] - dp[i - 1][j - i]).rem_euclid(1_000_000_007);
                }
            }
        }

        dp[n][k]
    }
}
```
