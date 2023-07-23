import math


class Solution:
    def smallestGoodBase(self, n: str) -> str:
        n = int(n)

        for x in range(math.ceil(math.log2(n)), 1, -1):
            l, r = 2, n - 1

            while l <= r:
                k = (l + r) // 2

                if k ** x - 1 == n * (k - 1):
                    return str(k)
                elif k ** x - 1 < n * (k - 1):
                    l = k + 1
                else:
                    r = k - 1
