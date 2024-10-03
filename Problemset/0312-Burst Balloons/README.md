# 312. Burst Balloons
You are given `n` balloons, indexed from `0` to `n - 1`. Each balloon is painted with a number on it represented by an array `nums`. You are asked to burst all the balloons.

If you burst the <code>i<sup>th</sup></code> balloon, you will get `nums[i - 1] * nums[i] * nums[i + 1]` coins. If `i - 1` or `i + 1` goes out of bounds of the array, then treat it as if there is a balloon with a `1` painted on it.

Return *the maximum coins you can collect by bursting the balloons wisely*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,1,5,8]
<strong>Output:</strong> 167
<strong>Explanation:</strong>
nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,5]
<strong>Output:</strong> 10
</pre>

#### Constraints:
* `n == nums.length`
* `1 <= n <= 300`
* `0 <= nums[i] <= 100`

## Solutions (Python)

### 1. Solution
```Python
from functools import cache


class Solution:
    def maxCoins(self, nums: List[int]) -> int:
        @cache
        def subarrayMaxCoins(i: int, j: int) -> int:
            if i >= j:
                return 0

            ret = 0

            for k in range(i, j):
                coins = nums[k]
                if i > 0:
                    coins *= nums[i - 1]
                if j < len(nums):
                    coins *= nums[j]
                coins += subarrayMaxCoins(i, k) + subarrayMaxCoins(k + 1, j)
                ret = max(ret, coins)

            return ret

        return subarrayMaxCoins(0, len(nums))
```
