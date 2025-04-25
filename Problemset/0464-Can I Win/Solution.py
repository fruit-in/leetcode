from functools import cache


class Solution:
    def canIWin(self, maxChoosableInteger: int, desiredTotal: int) -> bool:
        @cache
        def canIWinWithUsed(usedmask: int) -> bool:
            total = sum(i + 1 for i in range(maxChoosableInteger)
                        if (usedmask >> i) & 1 == 1)

            for i in range(maxChoosableInteger):
                if (usedmask >> i) & 1 == 0:
                    if total + i + 1 >= desiredTotal or not canIWinWithUsed(usedmask | (1 << i)):
                        return True

            return False

        return sum(range(1, maxChoosableInteger + 1)) >= desiredTotal and canIWinWithUsed(0)
