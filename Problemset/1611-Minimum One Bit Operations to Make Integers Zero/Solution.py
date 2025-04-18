from functools import cache


class Solution:
    @cache
    def minimumOneZerosOperations(self, n: int, i: int) -> int:
        if i == 0:
            return 1 - n

        if (n >> i) & 1 == 1:
            return self.minimumOneBitOperations(n & ((1 << i) - 1))
        else:
            return 1 + self.minimumOneZerosOperations(n, i - 1) + self.minimumOneBitOperations(1 << (i - 1))

    @cache
    def minimumOneBitOperations(self, n: int) -> int:
        if n <= 1:
            return n

        for i in range(29, 0, -1):
            if (n >> i) & 1 == 1:
                return 1 + self.minimumOneZerosOperations(n & ((1 << i) - 1), i - 1) + self.minimumOneBitOperations(1 << (i - 1))
