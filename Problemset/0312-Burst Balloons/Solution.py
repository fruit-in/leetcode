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
