# 2576. Find the Maximum Number of Marked Indices
You are given a **0-indexed** integer array `nums`.

Initially, all of the indices are unmarked. You are allowed to make this operation any number of times:
* Pick two **different unmarked** indices `i` and `j` such that `2 * nums[i] <= nums[j]`, then mark `i` and `j`.

Return *the maximum possible number of marked indices in `nums` using the above operation any number of times*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,5,2,4]
<strong>Output:</strong> 2
<strong>Explanation:</strong> In the first operation: pick i = 2 and j = 1, the operation is allowed because 2 * nums[2] <= nums[1]. Then mark index 2 and 1.
It can be shown that there's no other valid operation so the answer is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [9,2,5,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong> In the first operation: pick i = 3 and j = 0, the operation is allowed because 2 * nums[3] <= nums[0]. Then mark index 3 and 0.
In the second operation: pick i = 1 and j = 2, the operation is allowed because 2 * nums[1] <= nums[2]. Then mark index 1 and 2.
Since there is no other operation, the answer is 4.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [7,6,8]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no valid operation to do, so the answer is 0.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def maxNumOfMarkedIndices(self, nums: List[int]) -> int:
        n = len(nums)
        lo = n // 2
        nums.sort()

        for i in range(n // 2):
            lo = bisect_left(nums, 2 * nums[i], lo=lo) + 1
            if lo > n:
                return i * 2

        return n // 2 * 2
```
