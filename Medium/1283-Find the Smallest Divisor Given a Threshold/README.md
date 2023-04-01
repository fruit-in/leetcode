# 1283. Find the Smallest Divisor Given a Threshold
Given an array of integers `nums` and an integer `threshold`, we will choose a positive integer `divisor`, divide all the array by it, and sum the division's result. Find the **smallest** `divisor` such that the result mentioned above is less than or equal to `threshold`.

Each result of the division is rounded to the nearest integer greater than or equal to that element. (For example: `7/3 = 3` and `10/2 = 5`).

The test cases are generated so that there will be an answer.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,5,9], threshold = 6
<strong>Output:</strong> 5
<strong>Explanation:</strong> We can get a sum to 17 (1+2+5+9) if the divisor is 1.
If the divisor is 4 we can get a sum of 7 (1+1+2+3) and if the divisor is 5 the sum will be 5 (1+1+1+2).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [44,22,33,11,1], threshold = 5
<strong>Output:</strong> 44
</pre>

#### Constraints:
* <code>1 <= nums.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>
* <code>nums.length <= threshold <= 10<sup>6</sup></code>

## Solutions (Python)

### 1. Solution
```Python
import bisect
import math


class Solution:
    def smallestDivisor(self, nums: List[int], threshold: int) -> int:
        divisors = list(range(1, max(nums) + 1))

        return 1 + bisect.bisect_left(
            divisors,
            True,
            key=lambda x: sum(math.ceil(num / x) for num in nums) <= threshold
        )
```
