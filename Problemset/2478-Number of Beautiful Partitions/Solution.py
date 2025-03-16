from functools import cache


class Solution:
    def beautifulPartitions(self, s: str, k: int, minLength: int) -> int:
        @cache
        def subPartitions(i: int, k: int) -> int:
            if k == 0:
                return i == len(s)
            if len(s) - i < k * minLength or s[i] not in "2357":
                return 0

            ret = 0

            for j in range(i + minLength, len(s) - (k - 1) * minLength + 1):
                if s[j - 1] not in "2357":
                    ret = (ret + subPartitions(j, k - 1)) % 1000000007

            return ret

        if s[-1] in "2357":
            return 0

        return subPartitions(0, k)
