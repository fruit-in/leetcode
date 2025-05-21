# 491. Non-decreasing Subsequences
Given an integer array `nums`, return *all the different possible non-decreasing subsequences of the given array with at least two elements*. You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,6,7,7]
<strong>Output:</strong> [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,4,3,2,1]
<strong>Output:</strong> [[4,4]]
</pre>

#### Constraints:
* `1 <= nums.length <= 15`
* `-100 <= nums[i] <= 100`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findSubsequences(self, nums: List[int]) -> List[List[int]]:
        def dfs(i: int) -> None:
            if i == len(nums):
                if len(sub) >= 2:
                    subsequences.add(tuple(sub))
                return

            if not sub or nums[i] >= sub[-1]:
                sub.append(nums[i])
                dfs(i + 1)
                sub.pop()
            dfs(i + 1)

        sub = []
        subsequences = set()
        dfs(0)

        return [list(sub) for sub in subsequences]
```
