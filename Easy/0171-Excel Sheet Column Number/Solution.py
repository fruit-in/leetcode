class Solution:
    def titleToNumber(self, s: str) -> int:
        ret = 0
        for c in s:
            ret *= 26
            ret += ord(c) - 64
        return ret
