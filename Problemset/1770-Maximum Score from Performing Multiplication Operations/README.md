# 1770. Maximum Score from Performing Multiplication Operations
You are given two **0-indexed** integer arrays `nums` and `multipliers` of size `n` and `m` respectively, where `n >= m`.

You begin with a score of `0`. You want to perform **exactly** `m` operations. On the <code>i<sup>th</sup></code> operation (**0-indexed**) you will:
* Choose one integer `x` from **either the start or the end** of the array `nums`.
* Add `multipliers[i] * x` to your score.
    * Note that `multipliers[0]` corresponds to the first operation, `multipliers[1]` to the second operation, and so on.
* Remove `x` from `nums`.

Return *the **maximum** score after performing* `m` *operations*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3], multipliers = [3,2,1]
<strong>Output:</strong> 14
<strong>Explanation:</strong> An optimal solution is as follows:
- Choose from the end, [1,2,3], adding 3 * 3 = 9 to the score.
- Choose from the end, [1,2], adding 2 * 2 = 4 to the score.
- Choose from the end, [1], adding 1 * 1 = 1 to the score.
The total score is 9 + 4 + 1 = 14.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-5,-3,-3,-2,7,1], multipliers = [-10,-5,3,4,6]
<strong>Output:</strong> 102
<strong>Explanation:</strong> An optimal solution is as follows:
- Choose from the start, [-5,-3,-3,-2,7,1], adding -5 * -10 = 50 to the score.
- Choose from the start, [-3,-3,-2,7,1], adding -3 * -5 = 15 to the score.
- Choose from the start, [-3,-2,7,1], adding -3 * 3 = -9 to the score.
- Choose from the end, [-2,7,1], adding 1 * 4 = 4 to the score.
- Choose from the end, [-2,7], adding 7 * 6 = 42 to the score.
The total score is 50 + 15 - 9 + 4 + 42 = 102.
</pre>

#### Constraints:
* `n == nums.length`
* `m == multipliers.length`
* `1 <= m <= 300`
* <code>m <= n <= 10<sup>5</sup></code>
* `-1000 <= nums[i], multipliers[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let (n, m) = (nums.len(), multipliers.len());
        let mut dp = vec![vec![i32::MIN; m + 1]; m + 1];
        dp[0][0] = 0;

        for i in 1..=m {
            for j in 0..i {
                dp[j + 1][i - 1 - j] =
                    dp[j + 1][i - 1 - j].max(dp[j][i - 1 - j] + multipliers[i - 1] * nums[j]);
                dp[j][i - j] =
                    dp[j][i - j].max(dp[j][i - 1 - j] + multipliers[i - 1] * nums[n + j - i]);
            }
        }

        (0..=m).map(|i| dp[i][m - i]).max().unwrap()
    }
}
```
