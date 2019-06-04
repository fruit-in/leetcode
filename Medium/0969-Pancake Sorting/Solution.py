class Solution:
    def pancakeSort(self, A: List[int]) -> List[int]:
        finalA = sorted(A)
        ks = []
        n = len(A)
        for n in range(len(A), 0, -1):
            if A == finalA:
                break
            index = A.index(n)
            if index == n - 1:
                continue
            if index != 0:
                A[:index + 1] = A[index::-1]
                ks.append(index + 1)
            A[:n] = A[n - 1::-1]
            ks.append(n)
        return ks
