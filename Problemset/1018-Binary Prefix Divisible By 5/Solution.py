class Solution:
    def prefixesDivBy5(self, A: List[int]) -> List[bool]:
        n = 0
        for i in range(len(A)):
            n = (2 * n + A[i]) % 5
            A[i] = n == 0
        return A
