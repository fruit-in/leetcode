# 493. Reverse Pairs
Given an integer array `nums`, return *the number of **reverse pairs** in the array*.

A **reverse pair** is a pair `(i, j)` where:

* `0 <= i < j < nums.length` and
* `nums[i] > 2 * nums[j]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,2,3,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The reverse pairs are:
(1, 4) --> nums[1] = 3, nums[4] = 1, 3 > 2 * 1
(3, 4) --> nums[3] = 3, nums[4] = 1, 3 > 2 * 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,4,3,5,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The reverse pairs are:
(1, 4) --> nums[1] = 4, nums[4] = 1, 4 > 2 * 1
(2, 4) --> nums[2] = 3, nums[4] = 1, 3 > 2 * 1
(3, 4) --> nums[3] = 5, nums[4] = 1, 5 > 2 * 1
</pre>

#### Constraints:
* <code>1 <= nums.length <= 5 * 10<sup>4</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>

## Solutions (Python)

### 1. Solution
```Python
from sortedcontainers import SortedList


class Solution:
    def reversePairs(self, nums: List[int]) -> int:
        sortednums = SortedList()
        ret = 0

        for num in nums:
            ret += len(sortednums) - sortednums.bisect_right(2 * num)
            sortednums.add(num)

        return ret
```
