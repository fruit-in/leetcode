from functools import cache


class Solution:
    def minCost(self, n: int, cuts: List[int]) -> int:
        @cache
        def subStickMinCost(i: int, j: int) -> int:
            if i > j:
                return 0

            left, right = 0, n
            if i > 0:
                left = cuts[i - 1]
            if j < len(cuts) - 1:
                right = cuts[j + 1]

            return right - left + min(subStickMinCost(i, k - 1) + subStickMinCost(k + 1, j) for k in range(i, j + 1))

        cuts.sort()

        return subStickMinCost(0, len(cuts) - 1)
