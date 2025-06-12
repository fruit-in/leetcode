class Solution:
    def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
        n = len(matrix)
        r, c = n - 1, 0
        countle = 0
        countge = 0

        while countle < k or countge < n * n - k + 1:
            countle = sum(bisect.bisect(
                matrix[i], matrix[r][c]) for i in range(n))
            countge = sum(
                n - bisect.bisect(matrix[i], matrix[r][c] - 1) for i in range(n))
            if countle < k:
                c += 1
            if countge < n * n - k + 1:
                r -= 1

        return matrix[r][c]
