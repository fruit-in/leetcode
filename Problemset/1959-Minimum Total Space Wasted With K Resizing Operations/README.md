# 1959. Minimum Total Space Wasted With K Resizing Operations
You are currently designing a dynamic array. You are given a **0-indexed** integer array `nums`, where `nums[i]` is the number of elements that will be in the array at time `i`. In addition, you are given an integer `k`, the **maximum** number of times you can **resize** the array (to **any** size).

The size of the array at time `t`, <code>size<sub>t</sub></code>, must be at least `nums[t]` because there needs to be enough space in the array to hold all the elements. The **space wasted** at time `t` is defined as <code>size<sub>t</sub> - nums[t]</code>, and the **total** space wasted is the **sum** of the space wasted across every time `t` where `0 <= t < nums.length`.

Return *the **minimum total space wasted** if you can resize the array at most* `k` *times*.

**Note:** The array can have **any size** at the start and does **not** count towards the number of resizing operations.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10,20], k = 0
<strong>Output:</strong> 10
<strong>Explanation:</strong> size = [20,20].
We can set the initial size to be 20.
The total wasted space is (20 - 10) + (20 - 20) = 10.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [10,20,30], k = 1
<strong>Output:</strong> 10
<strong>Explanation:</strong> size = [20,20,30].
We can set the initial size to be 20 and resize to 30 at time 2.
The total wasted space is (20 - 10) + (20 - 20) + (30 - 30) = 10.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [10,20,15,30,20], k = 2
<strong>Output:</strong> 15
<strong>Explanation:</strong> size = [10,20,20,30,30].
We can set the initial size to 10, resize to 20 at time 1, and resize to 30 at time 3.
The total wasted space is (10 - 10) + (20 - 20) + (20 - 15) + (30 - 30) + (30 - 20) = 15.
</pre>

#### Constraints:
* `1 <= nums.length <= 200`
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* `0 <= k <= nums.length - 1`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def minSpaceWastedKResizing(self, nums: List[int], k: int) -> int:
        @cache
        def dfs(i: int, k: int) -> int:
            if i == len(nums):
                return 0
            if k == 1:
                return max(nums[i:]) * (len(nums) - i) - sum(nums[i:])

            maxnum = 0
            numssum = 0
            ret = inf

            for j in range(i, len(nums)):
                maxnum = max(maxnum, nums[j])
                numssum += nums[j]
                ret = min(ret, maxnum * (j - i + 1) -
                          numssum + dfs(j + 1, k - 1))

            return ret

        return dfs(0, k + 1)
```
