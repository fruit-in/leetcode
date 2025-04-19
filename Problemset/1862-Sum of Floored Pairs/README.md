# 1862. Sum of Floored Pairs
Given an integer array `nums`, return the sum of `floor(nums[i] / nums[j])` for all pairs of indices `0 <= i, j < nums.length` in the array. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

The `floor()` function returns the integer part of the division.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,5,9]
<strong>Output:</strong> 10
<strong>Explanation:</strong>
floor(2 / 5) = floor(2 / 9) = floor(5 / 9) = 0
floor(2 / 2) = floor(5 / 5) = floor(9 / 9) = 1
floor(5 / 2) = 2
floor(9 / 2) = 4
floor(9 / 5) = 1
We calculate the floor of the division for every pair of indices in the array then sum them up.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [7,7,7,7,7,7,7]
<strong>Output:</strong> 49
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def sumOfFlooredPairs(self, nums: List[int]) -> int:
        prevsum = 0
        ret = 0

        nums.sort()

        for j in range(len(nums)):
            if j > 0 and nums[j] == nums[j - 1]:
                ret = (ret + prevsum) % 1000000007
                continue

            i = bisect.bisect(nums, nums[j] - 1)
            prevsum = 0

            for x in range(1, nums[-1] // nums[j] + 1):
                k = bisect.bisect(nums, nums[j] * (x + 1) - 1, lo=i)
                prevsum = (prevsum + x * (k - i)) % 1000000007
                i = k

            ret = (ret + prevsum) % 1000000007

        return ret
```
