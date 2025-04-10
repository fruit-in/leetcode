# 1955. Count Number of Special Subsequences
A sequence is **special** if it consists of a **positive** number of `0`s, followed by a **positive** number of `1`s, then a **positive** number of `2`s.

* For example, `[0,1,2]` and `[0,0,1,1,1,2]` are special.
* In contrast, `[2,1,0]`, `[1]`, and `[0,1,2,0]` are not special.

Given an array `nums` (consisting of **only** integers `0`, `1`, and `2`), return *the **number of different subsequences** that are special*. Since the answer may be very large, **return it modulo** <code>10<sup>9</sup> + 7</code>.

A **subsequence** of an array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements. Two subsequences are **different** if the **set of indices** chosen are different.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,2,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The special subsequences are bolded [0,1,2,2], [0,1,2,2], and [0,1,2,2].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,2,0,0]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no special subsequences in [2,2,0,0].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [0,1,2,0,1,2]
<strong>Output:</strong> 7
<strong>Explanation:</strong> The special subsequences are bolded:
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
- [0,1,2,0,1,2]
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `0 <= nums[i] <= 2`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_special_subsequences(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut dp = vec![[0; 3]; nums.len() + 1];

        for i in 0..nums.len() {
            dp[i + 1] = dp[i];
            match nums[i] {
                0 => dp[i + 1][0] = (dp[i][0] * 2 + 1) % MOD,
                1 => dp[i + 1][1] = (dp[i][1] * 2 % MOD + dp[i][0]) % MOD,
                _ => dp[i + 1][2] = (dp[i][2] * 2 % MOD + dp[i][1]) % MOD,
            }
        }

        dp[nums.len()][2]
    }
}
```
