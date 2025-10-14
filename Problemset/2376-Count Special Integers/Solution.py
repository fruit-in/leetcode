class Solution:
    def countSpecialNumbers(self, n: int) -> int:
        @cache
        def countRemaining(s: str, mask: int) -> int:
            if s == "":
                return 1

            ret = 0

            for i in range(int(s[0])):
                if i == 0 and mask == 0:
                    ret += 9 * sum(362880 // factorial(j)
                                   for j in range(11 - len(s), 10)) + 1
                elif (mask >> i) & 1 == 0:
                    remain = 9 - mask.bit_count()
                    ret += prod(range(remain, remain - len(s) + 1, -1))
            if (mask >> int(s[0])) & 1 == 0:
                ret += countRemaining(s[1:], mask | (1 << int(s[0])))

            return ret

        return countRemaining(str(n), 0) - 1
