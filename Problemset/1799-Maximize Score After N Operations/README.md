# 1799. Maximize Score After N Operations
You are given `nums`, an array of positive integers of size `2 * n`. You must perform `n` operations on this array.

In the <code>i<sup>th</sup></code> operation **(1-indexed)**, you will:
* Choose two elements, `x` and `y`.
* Receive a score of `i * gcd(x, y)`.
* Remove `x` and `y` from `nums`.

Return *the maximum score you can receive after performing* `n` *operations*.

The function `gcd(x, y)` is the greatest common divisor of `x` and `y`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The optimal choice of operations is:
(1 * gcd(1, 2)) = 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,4,6,8]
<strong>Output:</strong> 11
<strong>Explanation:</strong> The optimal choice of operations is:
(1 * gcd(3, 6)) + (2 * gcd(4, 8)) = 3 + 8 = 11
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5,6]
<strong>Output:</strong> 14
<strong>Explanation:</strong> The optimal choice of operations is:
(1 * gcd(1, 5)) + (2 * gcd(2, 4)) + (3 * gcd(3, 6)) = 1 + 4 + 9 = 14
</pre>

#### Constraints:
* `1 <= n <= 7`
* `nums.length == 2 * n`
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Python)

### 1. Solution
```Python
import math
from functools import cache


class Solution:
    def maxScore(self, nums: List[int]) -> int:
        @cache
        def gcd(x: int, y: int) -> int:
            return math.gcd(x, y)

        n = len(nums) // 2
        pairmask = [[] for _ in range(n + 1)]
        dp = [0] * (1 << len(nums))

        for num in range(len(dp)):
            if bin(num).count('1') % 2 == 0:
                pairmask[bin(num).count('1') // 2].append(num)

        for i in range(1, n + 1):
            for j in range(len(nums)):
                x = nums[j]
                for k in range(j + 1, len(nums)):
                    y = nums[k]
                    for prevmask in pairmask[i - 1]:
                        mask = (1 << j) | (1 << k)
                        if prevmask & mask == 0:
                            dp[prevmask | mask] = max(
                                dp[prevmask | mask], dp[prevmask] + i * gcd(x, y))

        return dp[-1]
```
