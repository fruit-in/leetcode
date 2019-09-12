# 35. 搜索插入位置
给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

你可以假设数组中无重复元素。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,3,5,6], 5
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,3,5,6], 2
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [1,3,5,6], 7
<strong>输出:</strong> 4
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> [1,3,5,6], 0
<strong>输出:</strong> 0
</pre>

## 题解 (Python)

### 1. 线性扫描
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

### 2. 二分查找
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
