class Solution:
    def countSegments(self, s: str) -> int:
        cnt = 0

        for i in range(len(s)):
            if not s[i].isspace() and (i == 0 or s[i - 1].isspace()):
                cnt += 1

        return cnt
