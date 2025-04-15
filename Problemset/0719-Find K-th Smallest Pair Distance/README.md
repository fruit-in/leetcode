# 719. Find K-th Smallest Pair Distance
The **distance of a pair** of integers `a` and `b` is defined as the absolute difference between `a` and `b`.

Given an integer array `nums` and an integer `k`, return *the* <code>k<sup>th</sup></code> *smallest **distance among all the pairs*** `nums[i]` *and* `nums[j]` *where* `0 <= i < j < nums.length`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,1], k = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> Here are all the pairs:
(1,3) -> 2
(1,1) -> 0
(3,1) -> 2
Then the 1st smallest distance pair is (1,1), and its distance is 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1], k = 2
<strong>Output:</strong> 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,6,1], k = 3
<strong>Output:</strong> 5
</pre>

#### Constraints:
* `n == nums.length`
* <code>2 <= n <= 10<sup>4</sup></code>
* <code>0 <= nums[i] <= 10<sup>6</sup></code>
* `1 <= k <= n * (n - 1) / 2`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def smallestDistancePair(self, nums: List[int], k: int) -> int:
        nums.sort()

        lo = 0
        hi = nums[-1] - nums[0]

        while lo < hi:
            mid = (lo + hi) // 2
            count = 0

            for i in range(len(nums)):
                j = bisect.bisect(nums, nums[i] + mid, lo=i + 1)
                count += j - i - 1

            if count < k:
                lo = mid + 1
            else:
                hi = mid

        return hi
```
