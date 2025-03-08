from functools import cache


class Solution:
    def minScoreTriangulation(self, values: List[int]) -> int:
        @cache
        def subMinScore(i: int, j: int) -> int:
            return min((values[i] * values[j] * values[k] + subMinScore(i, k) + subMinScore(k, j) for k in range(i + 1, j)), default=0)

        return subMinScore(0, len(values) - 1)
