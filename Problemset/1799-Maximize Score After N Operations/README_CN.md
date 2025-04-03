# 1799. N 次操作后的最大分数和
给你 `nums` ，它是一个大小为 `2 * n` 的正整数数组。你必须对这个数组执行 `n` 次操作。

在第 `i` 次操作时（操作编号从 **1** 开始），你需要：
* 选择两个元素 `x` 和 `y` 。
* 获得分数 `i * gcd(x, y)` 。
* 将 `x` 和 `y` 从 `nums` 中删除。

请你返回 `n` 次操作后你能获得的分数和最大为多少。

函数 `gcd(x, y)` 是 `x` 和 `y` 的最大公约数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 最优操作是：
(1 * gcd(1, 2)) = 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,4,6,8]
<strong>输出:</strong> 11
<strong>解释:</strong> 最优操作是：
(1 * gcd(3, 6)) + (2 * gcd(4, 8)) = 3 + 8 = 11
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5,6]
<strong>输出:</strong> 14
<strong>解释:</strong> 最优操作是：
(1 * gcd(1, 5)) + (2 * gcd(2, 4)) + (3 * gcd(3, 6)) = 1 + 4 + 9 = 14
</pre>

#### 提示:
* `1 <= n <= 7`
* `nums.length == 2 * n`
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Python)

### 1. 题解
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
