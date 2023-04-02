import bisect


class Solution:
    def maxSideLength(self, mat: List[List[int]], threshold: int) -> int:
        m, n = len(mat), len(mat[0])
        prefixsum = [[0] * n for _ in range(m)]
        ret = 0

        for i in range(m):
            for j in range(n):
                prefixsum[i][j] = mat[i][j]
                prefixsum[i][j] += prefixsum[i - 1][j] if i > 0 else 0
                prefixsum[i][j] += prefixsum[i][j - 1] if j > 0 else 0
                prefixsum[i][j] -= prefixsum[i - 1][j - 1] \
                    if i > 0 and j > 0 else 0

                lengths = list(range(1, min(i, j) + 2))
                length = bisect.bisect_right(
                    lengths, threshold, key=lambda k: self.f(prefixsum, i, j, k))
                ret = max(ret, length)

        return ret

    def f(self, prefixsum, i, j, k):
        s = prefixsum[i][j]
        s -= prefixsum[i - k][j] if i >= k else 0
        s -= prefixsum[i][j - k] if j >= k else 0
        s += prefixsum[i - k][j - k] if i >= k and j >= k else 0

        return s
