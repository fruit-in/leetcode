# 628. Maximum Product of Three Numbers
Given an integer array, find three numbers whose product is maximum and output the maximum product.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3]
<strong>Output:</strong> 6
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,3,4]
<strong>Output:</strong> 24
</pre>

#### Note:
1. The length of the given array will be in range [3,10<sup>4</sup>] and all elements are in the range [-1000, 1000].
2. Multiplication of any three numbers in the input won't exceed the range of 32-bit signed integer.

## Solutions (Python)

### 1. Sort
```Python3
class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        nums.sort()
        return nums[-1] * max(nums[-2] * nums[-3], nums[0] * nums[1])
```

### 2. Linear Scan
```Python3
class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        min1, min2 = 1000, 1000
        max1, max2, max3 = -1000, -1000, -1000

        for n in nums:
            if n <= min1:
                min2 = min1
                min1 = n
            elif n < min2:
                min2 = n

            if n >= max3:
                max1 = max2
                max2 = max3
                max3 = n
            elif n >= max2:
                max1 = max2
                max2 = n
            elif n > max1:
                max1 = n

        return max3 * max(min1 * min2, max1 * max2)
```
