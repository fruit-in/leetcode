# 996. Number of Squareful Arrays
An array is **squareful** if the sum of every pair of adjacent elements is a **perfect square**.

Given an integer array nums, return *the number of permutations of* `nums` *that are **squareful***.

Two permutations `perm1` and `perm2` are different if there is some index `i` such that `perm1[i] != perm2[i]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,17,8]
<strong>Output:</strong> 2
<strong>Explanation:</strong> [1,8,17] and [17,8,1] are the valid permutations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,2,2]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= nums.length <= 12`
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numSquarefulPerms(self, nums: List[int]) -> int:
        n = len(nums)
        neighbors = {x: set() for x in nums} | {None: set(nums)}
        remain = {x: 0 for x in nums}
        ret = 0

        for i in range(n):
            x = nums[i]
            remain[x] += 1
            for j in range(i + 1, n):
                y = nums[j]
                if int(math.sqrt(x + y)) ** 2 == x + y:
                    neighbors[x].add(y)
                    neighbors[y].add(x)

        def dfs(i: int, prev: Optional[int]) -> None:
            nonlocal ret
            if i == n:
                ret += 1
                return

            for x in neighbors[prev]:
                if remain[x] > 0:
                    remain[x] -= 1
                    dfs(i + 1, x)
                    remain[x] += 1

        dfs(0, None)

        return ret
```
