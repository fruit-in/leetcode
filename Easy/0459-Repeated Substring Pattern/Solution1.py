class Solution:
    def repeatedSubstringPattern(self, s: str) -> bool:
        length = len(s)
        for i in filter(lambda x: length % x == 0, range(1, length // 2 + 1)):
            if s[:i] * (length // i) == s:
                return True
        return False
