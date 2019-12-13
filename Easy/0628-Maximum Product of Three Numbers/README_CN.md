# 628. 三个数的最大乘积
给定一个整型数组，在数组中找出由三个数组成的最大乘积，并输出这个乘积。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3]
<strong>输出:</strong> 6
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,3,4]
<strong>输出:</strong> 24
</pre>

#### 注意:
1. 给定的整型数组长度范围是[3,10<sup>4</sup>]，数组中所有的元素范围是[-1000, 1000]。
2. 输入的数组中任意三个数的乘积不会超出32位有符号整数的范围。

## 题解 (Python)

### 1. 排序
```Python3
class Solution:
    def maximumProduct(self, nums: List[int]) -> int:
        nums.sort()
        return nums[-1] * max(nums[-2] * nums[-3], nums[0] * nums[1])
```

### 2. 线性查找
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
