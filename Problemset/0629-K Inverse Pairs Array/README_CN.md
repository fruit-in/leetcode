# 629. K 个逆序对数组
逆序对的定义如下：对于数组 `nums` 的第 `i` 个和第 `j` 个元素，如果满足 `0 <= i < j < nums.length` 且 `nums[i] > nums[j]`，则其为一个逆序对；否则不是。

给你两个整数 `n` 和 `k`，找出所有包含从 `1` 到 `n` 的数字，且恰好拥有 `k` 个 **逆序对** 的不同的数组的个数。由于答案可能很大，只需要返回对 <code>10<sup>9</sup> + 7</code> 取余的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, k = 0
<strong>输出:</strong> 1
<strong>解释:</strong>
只有数组 [1,2,3] 包含了从1到3的整数并且正好拥有 0 个逆序对。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, k = 1
<strong>输出:</strong> 2
<strong>解释:</strong>
数组 [1,3,2] 和 [2,1,3] 都有 1 个逆序对。
</pre>

#### 提示:
* `1 <= n <= 1000`
* `0 <= k <= 1000`

## 题解 (Rust)

### 1. 题解
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
