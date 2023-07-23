class Solution:
    def addToArrayForm(self, A: List[int], K: int) -> List[int]:
        A[-1] += K
        i = -1
        while A[i] > 9:
            if len(A) > -i:
                A[i - 1] += A[i] // 10
            else:
                A = [A[i] // 10] + A
            A[i] %= 10
            i -= 1
        return A
