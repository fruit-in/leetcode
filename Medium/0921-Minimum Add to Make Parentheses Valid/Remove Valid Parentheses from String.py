class Solution:
    def minAddToMakeValid(self, S: str) -> int:
        while S.count("()"):
            S = S.replace("()", "")
        return len(S)
