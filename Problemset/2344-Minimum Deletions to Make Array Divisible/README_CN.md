# 2344. 使数组可以被整除的最少删除次数
给你两个正整数数组 `nums` 和 `numsDivide` 。你可以从 `nums` 中删除任意数目的元素。

请你返回使 `nums` 中 **最小** 元素可以整除 `numsDivide` 中所有元素的 **最少** 删除次数。如果无法得到这样的元素，返回 `-1` 。

如果 `y % x == 0` ，那么我们说整数 `x` 整除 `y` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,2,4,3], numsDivide = [9,6,9,3,15]
<strong>输出:</strong> 2
<strong>解释:</strong>
[2,3,2,4,3] 中最小元素是 2 ，它无法整除 numsDivide 中所有元素。
我们从 nums 中删除 2 个大小为 2 的元素，得到 nums = [3,4,3] 。
[3,4,3] 中最小元素为 3 ，它可以整除 numsDivide 中所有元素。
可以证明 2 是最少删除次数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,3,6], numsDivide = [8,2,6,10]
<strong>输出:</strong> -1
<strong>解释:</strong>
我们想 nums 中的最小元素可以整除 numsDivide 中的所有元素。
没有任何办法可以达到这一目的。
</pre>

#### 提示:
* <code>1 <= nums.length, numsDivide.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], numsDivide[i] <= 10<sup>9</sup></code>

## 题解 (Python)

### 1. 题解
```Python
from math import gcd


class Solution:
    def minOperations(self, nums: List[int], numsDivide: List[int]) -> int:
        y = gcd(*numsDivide)

        for i, x in enumerate(sorted(nums)):
            if y % x == 0:
                return i
            if y < x:
                break

        return -1
```
