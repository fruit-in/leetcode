class Solution:
    def arrangeCoins(self, n: int) -> int:
        return int(((1 + 8 * n) ** .5 - 1) / 2)
