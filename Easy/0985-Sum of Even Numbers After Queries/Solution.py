class Solution:
    def sumEvenAfterQueries(self, A: List[int], queries: List[List[int]]) -> List[int]:
        result = []
        S = sum(n for n in A if n % 2 == 0)
        for val, index in queries:
            if A[index] % 2 == 0:
                S -= A[index]
            A[index] += val
            if A[index] % 2 == 0:
                S += A[index]
            result.append(S)
        return result
