class Solution:
    def countBinarySubstrings(self, s: str) -> int:
        prev, curr = 0, 0
        ret = 0

        for i in range(len(s)):
            curr += 1
            if i == len(s) - 1 or s[i] != s[i + 1]:
                ret += min(prev, curr)
                prev, curr = curr, 0

        return ret
