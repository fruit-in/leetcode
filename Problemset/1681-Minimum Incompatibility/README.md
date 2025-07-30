# 1681. Minimum Incompatibility
You are given an integer array `nums` and an integer `k`. You are asked to distribute this array into `k` subsets of **equal size** such that there are no two equal elements in the same subset.

A subset's **incompatibility** is the difference between the maximum and minimum elements in that array.

Return *the **minimum possible sum of incompatibilities** of the* `k` *subsets after distributing the array optimally, or return* `-1` *if it is not possible*.

A subset is a group integers that appear in the array with no particular order.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,1,4], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The optimal distribution of subsets is [1,2] and [1,4].
The incompatibility is (2-1) + (4-1) = 4.
Note that [1,1] and [2,4] would result in a smaller sum, but the first subset contains 2 equal elements.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [6,3,8,1,3,1,2,2], k = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The optimal distribution of subsets is [1,2], [2,3], [6,8], and [1,3].
The incompatibility is (2-1) + (3-2) + (8-6) + (3-1) = 6.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [5,3,3,6,3,3], k = 3
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is impossible to distribute nums into 3 subsets where no two elements are equal in the same subset.
</pre>

#### Constraints:
* `1 <= k <= nums.length <= 16`
* `nums.length` is divisible by `k`
* `1 <= nums[i] <= nums.length`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minimumIncompatibility(self, nums: List[int], k: int) -> int:
        if max(Counter(nums).values()) > k:
            return -1
        if k == len(nums):
            return 0
        if k == 1:
            return max(nums) - min(nums)

        def dfs(i: int) -> None:
            nonlocal incompatibilities, minincompatibilities

            if subsets[-1] != [] and incompatibilities >= minincompatibilities:
                return

            if i == len(nums):
                minincompatibilities = incompatibilities
                return

            for j in range(k):
                if len(subsets[j]) == size or (subsets[j] != [] and subsets[j][-1] == nums[i]) or (j > 0 and subsets[j] == subsets[j - 1]):
                    continue

                subsets[j].append(nums[i])
                if len(subsets[j]) == 1:
                    incompatibilities -= nums[i]
                elif len(subsets[j]) == size:
                    incompatibilities += nums[i]
                dfs(i + 1)
                subsets[j].pop()
                if len(subsets[j]) == 0:
                    incompatibilities += nums[i]
                elif len(subsets[j]) == size - 1:
                    incompatibilities -= nums[i]

        size = len(nums) // k
        subsets = [[] for _ in range(k)]
        incompatibilities = 0
        minincompatibilities = inf
        nums.sort()
        dfs(0)

        return minincompatibilities
```
