# 2470. Number of Subarrays With LCM Equal to K
Given an integer array `nums` and an integer `k`, return *the number of **subarrays** of* `nums` *where the least common multiple of the subarray's elements is* `k`.

A **subarray** is a contiguous non-empty sequence of elements within an array.

The **least common multiple of an array** is the smallest positive integer that is divisible by all the array elements.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,6,2,7,1], k = 6
<strong>Output:</strong> 4
<strong>Explanation:</strong> The subarrays of nums where 6 is the least common multiple of all the subarray's elements are:
- [3,6,2,7,1]
- [3,6,2,7,1]
- [3,6,2,7,1]
- [3,6,2,7,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3], k = 2
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no subarrays of nums where 2 is the least common multiple of all the subarray's elements.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `1 <= nums[i], k <= 1000`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def subarrayLCM(self, nums: List[int], k: int) -> int:
        ret = 0

        for i in range(len(nums)):
            x = 1
            for j in range(i, len(nums)):
                x = lcm(x, nums[j])
                if x == k:
                    ret += 1
                elif x > k:
                    break

        return ret
```
