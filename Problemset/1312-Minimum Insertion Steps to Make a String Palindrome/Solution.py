from functools import cache


class Solution:
    def minInsertions(self, s: str) -> int:
        @cache
        def minInsertionsSub(i: int, j: int) -> int:
            if i >= j:
                return 0

            ret = 1 + minInsertionsSub(i + 1, j)
            ret = min(ret, 1 + minInsertionsSub(i, j - 1))
            if s[i] == s[j]:
                ret = min(ret, minInsertionsSub(i + 1, j - 1))

            return ret

        return minInsertionsSub(0, len(s) - 1)
