# 698. Partition to K Equal Sum Subsets
Given an integer array `nums` and an integer `k`, return `true` if it is possible to divide this array into `k` non-empty subsets whose sums are all equal.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,3,2,3,5,2,1], k = 4
<strong>Output:</strong> true
<strong>Explanation:</strong> It is possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], k = 3
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= k <= nums.length <= 16`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* The frequency of each element is in the range `[1, 4]`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def canPartitionKSubsets(self, nums: List[int], k: int) -> bool:
        def dfs(i: int) -> bool:
            if i == len(nums):
                return True

            prevsets = set()

            for j in range(k):
                if subsets[j] not in prevsets and subsets[j] + nums[i] <= subsum:
                    prevsets.add(subsets[j])
                    subsets[j] += nums[i]
                    if dfs(i + 1):
                        return True
                    subsets[j] -= nums[i]

            return False

        if sum(nums) % k != 0:
            return False

        nums.sort()
        subsum = sum(nums) // k

        while nums and nums[-1] == subsum:
            nums.pop()
            k -= 1

        if k > 0 and (nums[-1] > subsum or nums[-1] + nums[0] > subsum):
            return False

        subsets = [0] * k
        nums.reverse()

        return dfs(0)
```
