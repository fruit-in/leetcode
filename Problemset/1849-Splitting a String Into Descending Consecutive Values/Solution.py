from functools import cache


class Solution:
    @cache
    def splitString(self, s: str, x: int = None) -> bool:
        if x is None:
            return any(self.splitString(s[i:], int(s[:i])) for i in range(1, len(s)))

        if s == "":
            return True

        return any(x - 1 == int(s[:i]) and self.splitString(s[i:], int(s[:i])) for i in range(1, len(s) + 1))
