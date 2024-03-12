# 805. Split Array With Same Average
You are given an integer array `nums`.

You should move each element of `nums` into one of the two arrays `A` and `B` such that `A` and `B` are non-empty, and `average(A) == average(B)`.

Return `true` if it is possible to achieve that and `false` otherwise.

**Note** that for an array `arr`, `average(arr)` is the sum of all the elements of `arr` over the length of `arr`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6,7,8]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,1]
<strong>Output:</strong> false
</pre>

#### Constraints:
* `1 <= nums.length <= 30`
* <code>0 <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def splitArraySameAverage(self, nums: List[int]) -> bool:
        if len(nums) < 2:
            return False

        s = sum(nums)
        sums = [set() for _ in range(len(nums) // 2 + 1)]
        sums[0].add(0)

        for num in nums:
            for i in range(len(sums) - 2, -1, -1):
                for x in sums[i]:
                    if (x + num) * len(nums) == s * (i + 1):
                        return True

                    sums[i + 1].add(x + num)

        return False
```
