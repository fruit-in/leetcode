# 813. Largest Sum of Averages
You are given an integer array `nums` and an integer `k`. You can partition the array into **at most** `k` non-empty adjacent subarrays. The **score** of a partition is the sum of the averages of each subarray.

Note that the partition must use every integer in `nums`, and that the score is not necessarily an integer.

Return *the maximum **score** you can achieve of all the possible partitions*. Answers within <code>10<sup>-6</sup></code> of the actual answer will be accepted.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [9,1,2,3,9], k = 3
<strong>Output:</strong> 20.00000
<strong>Explanation:</strong>
The best choice is to partition nums into [9], [1, 2, 3], [9]. The answer is 9 + (1 + 2 + 3) / 3 + 9 = 20.
We could have also partitioned nums into [9, 1], [2], [3, 9], for example.
That partition would lead to a score of 5 + 2 + 6 = 13, which is worse.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6,7], k = 4
<strong>Output:</strong> 20.50000
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* `1 <= k <= nums.length`

## Solutions (Python)

### 1. Solution
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
