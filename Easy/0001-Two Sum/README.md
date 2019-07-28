# 1. Two Sum
Given an array of integers, return **indices** of the two numbers such that they add up to a specific target.

You may assume that each input would have ***exactly*** one solution, and you may not use the same element twice.

#### Example:
<pre>
Given nums = [2, 7, 11, 15], target = 9,

Because nums[<strong>0</strong>] + nums[<strong>1</strong>] = 2 + 7 = 9,
return [<strong>0</strong>, <strong>1</strong>].
</pre>

## Solutions

### 1. Brute Force (Python3)
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for k, v in enumerate(nums):
            if target - v in nums[k + 1:]:
                return [k, k + 1 + nums[k + 1:].index(target - v)]
```

### 2. One Pass Hash Table (Python3)
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        s = {}
        for k, v in enumerate(nums):
            if target - v in s.keys():
                return [k, s[target - v]]
            s[v] = k
```

### 3. Two Pass Hash Table (Python3)
```Python3
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        s = {}
        for k, v in enumerate(nums):
            s[v] = k
        for k, v in enumerate(nums):
            if target - v in s.keys() and s[target - v] != k:
                return [k, s[target - v]]
```
