# 1770. 执行乘法运算的最大分数
给你两个长度分别 `n` 和 `m` 的整数数组 `nums` 和 `multipliers` ，其中 `n >= m` ，数组下标 **从 1 开始** 计数。

初始时，你的分数为 `0` 。你需要执行恰好 `m` 步操作。在第 `i` 步操作（**从 1 开始** 计数）中，需要：
* 选择数组 `nums` **开头处或者末尾处** 的整数 `x` 。
* 你获得 `multipliers[i] * x` 分，并累加到你的分数中。
* 将 `x` 从数组 `nums` 中移除。

在执行 `m` 步操作后，返回 **最大** 分数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3], multipliers = [3,2,1]
<strong>输出:</strong> 14
<strong>解释:</strong> 一种最优解决方案如下：
- 选择末尾处的整数 3 ，[1,2,3] ，得 3 * 3 = 9 分，累加到分数中。
- 选择末尾处的整数 2 ，[1,2] ，得 2 * 2 = 4 分，累加到分数中。
- 选择末尾处的整数 1 ，[1] ，得 1 * 1 = 1 分，累加到分数中。
总分数为 9 + 4 + 1 = 14 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-5,-3,-3,-2,7,1], multipliers = [-10,-5,3,4,6]
<strong>输出:</strong> 102
<strong>解释:</strong> 一种最优解决方案如下：
- 选择开头处的整数 -5 ，[-5,-3,-3,-2,7,1] ，得 -5 * -10 = 50 分，累加到分数中。
- 选择开头处的整数 -3 ，[-3,-3,-2,7,1] ，得 -3 * -5 = 15 分，累加到分数中。
- 选择开头处的整数 -3 ，[-3,-2,7,1] ，得 -3 * 3 = -9 分，累加到分数中。
- 选择末尾处的整数 1 ，[-2,7,1] ，得 1 * 4 = 4 分，累加到分数中。
- 选择末尾处的整数 7 ，[-2,7] ，得 7 * 6 = 42 分，累加到分数中。
总分数为 50 + 15 - 9 + 4 + 42 = 102 。
</pre>

#### 提示:
* `n == nums.length`
* `m == multipliers.length`
* `1 <= m <= 300`
* <code>m <= n <= 10<sup>5</sup></code>
* `-1000 <= nums[i], multipliers[i] <= 1000`

## 题解 (Rust)

### 1. 题解
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
