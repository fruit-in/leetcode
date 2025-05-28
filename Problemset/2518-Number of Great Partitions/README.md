# 2518. Number of Great Partitions
You are given an array `nums` consisting of **positive** integers and an integer `k`.

**Partition** the array into two ordered **groups** such that each element is in exactly **one** group. A partition is called great if the **sum** of elements of each group is greater than or equal to `k`.

Return *the number of **distinct** great partitions*. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

Two partitions are considered distinct if some element `nums[i]` is in different groups in the two partitions.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The great partitions are: ([1,2,3], [4]), ([1,3], [2,4]), ([1,4], [2,3]), ([2,3], [1,4]), ([2,4], [1,3]) and ([4], [1,2,3]).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,3,3], k = 4
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no great partitions for this array.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [6,6], k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can either put nums[0] in the first partition or in the second partition.
The great partitions will be ([6], [6]) and ([6], [6]).
</pre>

#### Constraints:
* `1 <= nums.length, k <= 1000`
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countPartitions(self, nums: List[int], k: int) -> int:
        if sum(nums) < k * 2:
            return 0

        MOD = 1000000007
        n = len(nums)
        dp = [[0] * (k + 1) for _ in range(n + 1)]
        dp[0][0] = 1
        ret = pow(2, n, MOD)

        for i in range(n):
            for j in range(k + 1):
                dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % MOD
                dp[i + 1][min(k, j + nums[i])] = (dp[i + 1]
                                                  [min(k, j + nums[i])] + dp[i][j]) % MOD

        for i in range(k):
            ret = (ret - dp[n][i] * 2) % MOD

        return ret
```
