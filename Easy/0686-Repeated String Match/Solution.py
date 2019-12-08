class Solution:
    def repeatedStringMatch(self, A: str, B: str) -> int:
        r = (len(B) - 1) // len(A) + 1
        if B in A * r:
            return r
        elif B in A * (r + 1):
            return r + 1
        return -1
