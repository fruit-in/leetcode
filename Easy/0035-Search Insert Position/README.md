# 35. Search Insert Position
Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You may assume no duplicates in the array.

#### Example 1:
<pre>
<strong>Input:</strong> [1,3,5,6], 5
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,3,5,6], 2
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [1,3,5,6], 7
<strong>Output:</strong> 4
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> [1,3,5,6], 0
<strong>Output:</strong> 0
</pre>

## Solutions (Python)

### 1. Linear Scan
```Python3
class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if not nums or target <= nums[0]:
            return 0
        if target > nums[-1]:
            return len(nums)
        for i in range(1, len(nums)):
            if target == nums[i] or (target > nums[i - 1] and target < nums[i]):
                return i
```

### 2. Binary Search
```Python3
class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        if not nums or target <= nums[0]:
            return 0
        if target > nums[-1]:
            return len(nums)

        left = 1
        right = len(nums) - 1
        while left <= right:
            mid = (left + right) // 2

            if target == nums[mid]:
                return mid
            elif target > nums[mid]:
                left = mid + 1
                if target < nums[left]:
                    return left
            elif target < nums[mid]:
                right = mid - 1
                if target > nums[right]:
                    return mid
```
