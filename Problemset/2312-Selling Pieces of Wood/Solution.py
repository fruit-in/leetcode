from functools import cache


class Solution:
    def sellingWood(self, m: int, n: int, prices: List[List[int]]) -> int:
        prices = {(h, w): price for h, w, price in prices}

        @cache
        def maxSelling(m: int, n: int) -> int:
            ret = prices.get((m, n), 0)

            for h in range(1, m):
                ret = max(ret, maxSelling(h, n) + maxSelling(m - h, n))
            for w in range(1, n):
                ret = max(ret, maxSelling(m, w) + maxSelling(m, n - w))

            return ret

        return maxSelling(m, n)
