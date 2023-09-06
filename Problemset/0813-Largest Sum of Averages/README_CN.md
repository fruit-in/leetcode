# 813. 最大平均值和的分组
给定数组 `nums` 和一个整数 `k` 。我们将给定的数组 `nums` 分成 **最多** `k` 个非空子数组，且数组内部是连续的 。 **分数** 由每个子数组内的平均值的总和构成。

注意我们必须使用 `nums` 数组中的每一个数进行分组，并且分数不一定需要是整数。

返回我们所能得到的最大 **分数** 是多少。答案误差在 <code>10<sup>-6</sup></code> 内被视为是正确的。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [9,1,2,3,9], k = 3
<strong>输出:</strong> 20.00000
<strong>解释:</strong>
nums 的最优分组是[9], [1, 2, 3], [9]. 得到的分数是 9 + (1 + 2 + 3) / 3 + 9 = 20.
我们也可以把 nums 分成[9, 1], [2], [3, 9].
这样的分组得到的分数为 5 + 2 + 6 = 13, 但不是最大值.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5,6,7], k = 4
<strong>输出:</strong> 20.50000
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* `1 <= k <= nums.length`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def largestSumOfAverages(self, nums: List[int], k: int) -> float:
        n = len(nums)
        prefixsum = [0] * (n + 1)
        dp = [[0] * (k + 1) for _ in range(n + 1)]

        for i in range(n):
            prefixsum[i + 1] = prefixsum[i] + nums[i]

        for i in range(n):
            for j in range(min(1, i), k):
                for s in range(1, n + 1 - i):
                    dp[i + s][j + 1] = max(dp[i + s][j + 1],
                                           dp[i][j] + (prefixsum[i + s] - prefixsum[i]) / s)

        return max(dp[n])
```
