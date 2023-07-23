# 665. Non-decreasing Array
Given an array with ```n``` integers, your task is to check if it could become non-decreasing by modifying **at most** ```1``` element.

We define an array is non-decreasing if ```array[i] <= array[i + 1]``` holds for every ```i``` (1 <= i < n). 

#### Example 1:
<pre>
<strong>Input:</strong> [4,2,3]
<strong>Output:</strong> True
<strong>Explanation:</strong> You could modify the first 4 to 1 to get a non-decreasing array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [4,2,1]
<strong>Output:</strong> False
<strong>Explanation:</strong> You can't get a non-decreasing array by modify at most one element.
</pre>

**Note:** The ```n``` belongs to [1, 10,000].

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def checkPossibility(self, nums: List[int]) -> bool:
        flag = False
        for i in range(len(nums) - 1):
            if nums[i] > nums[i + 1]:
                if flag:
                    return False
                flag = True
                if i > 0 and nums[i - 1] > nums[i + 1]:
                    nums[i + 1] = nums[i]
        return True
```
