class Solution:
    def checkString(self, s: str) -> bool:
        return all(s[i] <= s[i + 1] for i in range(len(s) - 1))
