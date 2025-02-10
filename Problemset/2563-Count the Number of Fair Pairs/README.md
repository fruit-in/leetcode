# 2563. Count the Number of Fair Pairs
Given a **0-indexed** integer array `nums` of size `n` and two integers `lower` and `upper`, return *the number of fair pairs*.

A pair `(i, j)` is **fair** if:
* `0 <= i < j < n`, and
* `lower <= nums[i] + nums[j] <= upper`

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,7,4,4,5], lower = 3, upper = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,7,9,2,5], lower = 11, upper = 11
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is a single fair pair: (2,3).
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums.length == n`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* <code>-10<sup>9</sup> <= lower <= upper <= 10<sup>9</sup></code>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countFairPairs(self, nums: List[int], lower: int, upper: int) -> int:
        nums.sort()
        ret = 0

        for i in range(len(nums) - 1):
            if nums[i] + nums[-1] < lower:
                continue
            if nums[i] + nums[i + 1] > upper:
                break

            j = max(bisect.bisect_left(nums, lower - nums[i]), i + 1)
            k = bisect.bisect(nums, upper - nums[i])
            ret += k - j

        return ret
```
