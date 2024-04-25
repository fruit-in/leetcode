from functools import lru_cache


class Solution:
    @lru_cache()
    def partition(self, s: str) -> List[List[str]]:
        if len(s) == 0:
            return [[]]

        ret = []

        for i in range(1, len(s) + 1):
            if s[:i] == s[:i][::-1]:
                ret.extend([[s[:i]] + x for x in self.partition(s[i:])])

        return ret
