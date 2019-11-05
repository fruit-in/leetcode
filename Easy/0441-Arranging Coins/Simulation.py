class Solution:
    def arrangeCoins(self, n: int) -> int:
        k = 1
        while True:
            n -= k
            if n < 0:
                return k - 1
            k += 1
