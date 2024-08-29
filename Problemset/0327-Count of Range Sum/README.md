# 327. Count of Range Sum
Given an integer array `nums` and two integers `lower` and `upper`, return *the number of range sums that lie in* `[lower, upper]` *inclusive*.

Range sum `S(i, j)` is defined as the sum of the elements in `nums` between indices `i` and `j` inclusive, where `i <= j`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-2,5,-1], lower = -2, upper = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> The three ranges are: [0,0], [2,2], and [0,2] and their respective sums are: -2, -1, 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0], lower = 0, upper = 0
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>
* <code>-10<sup>5</sup> <= lower <= upper <= 10<sup>5</sup></code>
* The answer is **guaranteed** to fit in a **32-bit** integer.

## Solutions (Python)

### 1. Solution
```Python
from sortedcontainers import SortedList


class Solution:
    def countRangeSum(self, nums: List[int], lower: int, upper: int) -> int:
        prefixsums = SortedList([0])
        prefixsum = 0
        ret = 0

        for i in range(len(nums)):
            prefixsum += nums[i]
            j = prefixsums.bisect_right(prefixsum - lower)
            k = prefixsums.bisect_left(prefixsum - upper)
            ret += j - k
            prefixsums.add(prefixsum)

        return ret
```
