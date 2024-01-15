# 315. Count of Smaller Numbers After Self
Given an integer array `nums`, return *an integer array* `counts` *where* `counts[i]` *is the number of smaller elements to the right of* `nums[i]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [5,2,6,1]
<strong>Output:</strong> [2,1,1,0]
<strong>Explanation:</strong>
To the right of 5 there are 2 smaller elements (2 and 1).
To the right of 2 there is only 1 smaller element (1).
To the right of 6 there is 1 smaller element (1).
To the right of 1 there is 0 smaller element.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-1]
<strong>Output:</strong> [0]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [-1,-1]
<strong>Output:</strong> [0,0]
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
```Python
from sortedcontainers import SortedList


class Solution:
    def countSmaller(self, nums: List[int]) -> List[int]:
        sortednums = SortedList()
        counts = [0] * len(nums)

        for i in range(len(nums) - 1, -1, -1):
            counts[i] = sortednums.bisect_left(nums[i])
            sortednums.add(nums[i])

        return counts
```
