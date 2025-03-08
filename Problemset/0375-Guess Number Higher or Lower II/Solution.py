from functools import cache


class Solution:
    @cache
    def getMoneyAmount(self, n: int, dollars=0) -> int:
        if n <= 1:
            return 0

        return min(x + dollars + max(self.getMoneyAmount(x - 1, dollars), self.getMoneyAmount(n - x, dollars + x)) for x in range(1, n + 1))
