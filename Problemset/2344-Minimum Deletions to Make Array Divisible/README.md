# 2344. Minimum Deletions to Make Array Divisible
You are given two positive integer arrays `nums` and `numsDivide`. You can delete any number of elements from `nums`.

Return *the **minimum** number of deletions such that the **smallest** element in* `nums` ***divides** all the elements of* `numsDivide`. If this is not possible, return `-1`.

Note that an integer `x` divides `y` if `y % x == 0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,3,2,4,3], numsDivide = [9,6,9,3,15]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The smallest element in [2,3,2,4,3] is 2, which does not divide all the elements of numsDivide.
We use 2 deletions to delete the elements in nums that are equal to 2 which makes nums = [3,4,3].
The smallest element in [3,4,3] is 3, which divides all the elements of numsDivide.
It can be shown that 2 is the minimum number of deletions needed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,3,6], numsDivide = [8,2,6,10]
<strong>Output:</strong> -1
<strong>Explanation:</strong>
We want the smallest element in nums to divide all the elements of numsDivide.
There is no way to delete elements from nums to allow this.
</pre>

#### Constraints:
* <code>1 <= nums.length, numsDivide.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], numsDivide[i] <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
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
