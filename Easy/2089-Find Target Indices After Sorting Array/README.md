# 2089. Find Target Indices After Sorting Array
You are given a **0-indexed** integer array `nums` and a target element `target`.

A **target index** is an index `i` such that `nums[i] == target`.

Return *a list of the target indices of* `nums` after *sorting* `nums` *in **non-decreasing** order*. If there are no target indices, return *an **empty** list*. The returned list must be sorted in **increasing** order.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,5,2,3], target = 2
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> After sorting, nums is [1,2,2,3,5].
The indices where nums[i] == 2 are 1 and 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,5,2,3], target = 3
<strong>Output:</strong> [3]
<strong>Explanation:</strong> After sorting, nums is [1,2,2,3,5].
The index where nums[i] == 3 is 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,5,2,3], target = 5
<strong>Output:</strong> [4]
<strong>Explanation:</strong> After sorting, nums is [1,2,2,3,5].
The index where nums[i] == 5 is 4.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i], target <= 100`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def targetIndices(self, nums: List[int], target: int) -> List[int]:
        nums.sort()

        return list(range(bisect_left(nums, target), bisect_right(nums, target)))
```
