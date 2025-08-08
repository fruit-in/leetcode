# 2447. Number of Subarrays With GCD Equal to K
Given an integer array `nums` and an integer `k`, return *the number of **subarrays** of* `nums` *where the greatest common divisor of the subarray's elements is* `k`.

A **subarray** is a contiguous non-empty sequence of elements within an array.

The **greatest common divisor of an array** is the largest integer that evenly divides all the array elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [9,3,1,2,6,3], k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong> The subarrays of nums where 3 is the greatest common divisor of all the subarray's elements are:
- [9,3,1,2,6,3]
- [9,3,1,2,6,3]
- [9,3,1,2,6,3]
- [9,3,1,2,6,3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4], k = 7
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no subarrays of nums where 7 is the greatest common divisor of all the subarray's elements.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i], k <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def subarrayGCD(self, nums: List[int], k: int) -> int:
        ret = 0

        for i in range(len(nums)):
            maxgcd = nums[i]

            for j in range(i, len(nums)):
                maxgcd = gcd(maxgcd, nums[j])
                if maxgcd % k != 0:
                    break
                elif maxgcd == k:
                    ret += 1

        return ret
```
