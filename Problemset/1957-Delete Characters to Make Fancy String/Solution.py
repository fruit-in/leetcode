class Solution:
    def makeFancyString(self, s: str) -> str:
        ret = ""

        for c in s:
            if len(ret) < 2 or ret[-1] != c or ret[-2] != c:
                ret += c

        return ret
