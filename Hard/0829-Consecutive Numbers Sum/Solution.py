class Solution:
    def consecutiveNumbersSum(self, n: int) -> int:
        ret = 0

        for x in range(1, int((2 * n) ** .5) + 1):
            if (n + (x - 1) * x // 2) % x == 0 and (n + (x - 1) * x // 2) // x > 0:
                ret += 1

        return ret
