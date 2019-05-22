class Solution:
    def buddyStrings(self, A: str, B: str) -> bool:
        if len(A) != len(B):
            return False
        if A == B and len(set(A)) != len(A):
            return True
        a, b = '', ''
        for k, v in enumerate(A):
            if v != B[k]:
                a += v
                b += B[k]
            if len(a) > 2:
                return False
        return len(a) == 2 and a == b[::-1]
