# 2597. The Number of Beautiful Subsets
You are given an array `nums` of positive integers and a **positive** integer `k`.

A subset of `nums` is **beautiful** if it does not contain two integers with an absolute difference equal to `k`.

Return *the number of **non-empty beautiful** subsets of the array* `nums`.

A **subset** of `nums` is an array that can be obtained by deleting some (possibly none) elements from `nums`. Two subsets are different if and only if the chosen indices to delete are different.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,4,6], k = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> The beautiful subsets of the array nums are: [2], [4], [6], [2, 6].
It can be proved that there are only 4 beautiful subsets in the array [2,4,6].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1], k = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong> The beautiful subset of the array nums is [1].
It can be proved that there is only 1 beautiful subset in the array [1].
</pre>

#### Constraints:
* `1 <= nums.length <= 18`
* `1 <= nums[i], k <= 1000`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def beautifulSubsets(self, nums: List[int], k: int) -> int:
        def dfs(i: int) -> None:
            nonlocal ret
            if i == len(nums):
                return

            j = bisect.bisect_left(subset, nums[i] - k)
            if j == len(subset) or subset[j] != nums[i] - k:
                subset.append(nums[i])
                ret += 1
                dfs(i + 1)
                subset.pop()
            dfs(i + 1)

        subset = []
        ret = 0
        nums.sort()

        dfs(0)

        return ret
```
