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
